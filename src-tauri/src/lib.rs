pub mod ff;
pub mod media;
pub mod models;
pub mod strings;

use ff::extract_streams;
use media::{detect_media_type, is_media_file};
use models::{StreamResult, StreamResultError};
use strings::transform_filename;
use std::path::Path;
use tracing::{debug, error, info, instrument, warn};

// detection, helpers moved to modules above

#[tauri::command]
#[instrument(skip(paths), fields(file_count = paths.len()))]
fn has_streams(paths: Vec<String>) -> Result<Vec<StreamResult>, StreamResultError> {
    info!(file_count = paths.len(), "Processing files for stream analysis");
    
    let results = paths
        .into_iter()
        .enumerate()
        .map(|(index, path_str)| {
            let path = Path::new(&path_str);
            let filename = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            // Log full path only on first occurrence, then use truncated filename
            if index == 0 {
                debug!(full_path = %path_str, filename = %filename, "Processing first file");
            } else {
                let truncated_name = transform_filename(&filename, 15);
                debug!(filename = %truncated_name, "Processing file");
            }

            // Check if file exists
            if !path.exists() {
                let truncated_name = transform_filename(&filename, 15);
                warn!(filename = %truncated_name, "File does not exist");
                return Err(StreamResultError {
                    filename: Some(filename),
                    reason: "File does not exist".to_string(),
                    error_type: "not_found".to_string(),
                });
            }

            // Check if it's a file (not directory)
            if !path.is_file() {
                let truncated_name = transform_filename(&filename, 15);
                warn!(filename = %truncated_name, "Path is not a file");
                return Err(StreamResultError {
                    filename: Some(filename),
                    reason: "Not a file (directory or other)".to_string(),
                    error_type: "not_file".to_string(),
                });
            }

            // Get file size
            let size = std::fs::metadata(&path_str)
                .map(|metadata| metadata.len())
                .unwrap_or(0);

            let truncated_name = transform_filename(&filename, 15);
            debug!(filename = %truncated_name, size = size, "File metadata retrieved");

            // Detect media type using magic numbers and fallback to extensions
            let media_type = detect_media_type(path);
            debug!(filename = %truncated_name, media_type = ?media_type, "Media type detected");

            // Only try to analyze media files with ffprobe
            if is_media_file(&media_type) {
                info!(filename = %truncated_name, media_type = ?media_type, "Analyzing media file with ffprobe");
                
                // Analyze with ffprobe
                match ffprobe::ffprobe(&path_str) {
                    Ok(info) => {
                        let streams = extract_streams(&info);
                        let duration = info
                            .format
                            .duration
                            .and_then(|dur_str| dur_str.parse::<f64>().ok());

                        info!(
                            filename = %truncated_name,
                            stream_count = streams.len(),
                            duration = ?duration,
                            "Successfully analyzed media file"
                        );

                        Ok(StreamResult {
                            filename,
                            path: path_str,
                            media_type,
                            duration,
                            size,
                            streams,
                        })
                    }
                    Err(err) => {
                        error!(filename = %truncated_name, error = %err, "Failed to analyze media file with ffprobe");
                        Err(StreamResultError {
                            filename: Some(filename),
                            reason: format!("Could not analyze media file: {err}"),
                            error_type: "analysis_failed".to_string(),
                        })
                    }
                }
            } else {
                debug!(filename = %truncated_name, media_type = ?media_type, "Skipping non-media file");
                // For non-media files, return an error indicating it's not a media file
                Err(StreamResultError {
                    filename: Some(filename),
                    reason: format!("Not a media file (detected as {media_type:?})"),
                    error_type: "not_media".to_string(),
                })
            }
        })
        .collect::<Result<Vec<_>, _>>();

    match &results {
        Ok(streams) => {
            info!(successful_files = streams.len(), "Successfully processed all files");
        }
        Err(_) => {
            warn!("Some files failed to process");
        }
    }

    results
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    info!("Initializing Tauri application");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![has_streams])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}