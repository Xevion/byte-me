mod ff;
mod media;
mod models;

use ff::extract_streams;
use media::{detect_media_type, is_media_file};
use models::{StreamResult, StreamResultError};
use std::path::Path;

// detection, helpers moved to modules above

#[tauri::command]
fn has_streams(paths: Vec<String>) -> Result<Vec<StreamResult>, StreamResultError> {
    paths
        .into_iter()
        .map(|path_str| {
            let path = Path::new(&path_str);
            let filename = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown")
                .to_string();

            // Check if file exists
            if !path.exists() {
                return Err(StreamResultError {
                    filename: Some(filename),
                    reason: "File does not exist".to_string(),
                    error_type: "not_found".to_string(),
                });
            }

            // Check if it's a file (not directory)
            if !path.is_file() {
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

            // Detect media type using magic numbers and fallback to extensions
            let media_type = detect_media_type(path);

            // Only try to analyze media files with ffprobe
            if is_media_file(&media_type) {
                // Analyze with ffprobe
                match ffprobe::ffprobe(&path_str) {
                    Ok(info) => {
                        let streams = extract_streams(&info);
                        let duration = info
                            .format
                            .duration
                            .and_then(|dur_str| dur_str.parse::<f64>().ok());

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
                        eprintln!("Could not analyze media file with ffprobe: {err:?}");
                        Err(StreamResultError {
                            filename: Some(filename),
                            reason: format!("Could not analyze media file: {err}"),
                            error_type: "analysis_failed".to_string(),
                        })
                    }
                }
            } else {
                // For non-media files, return an error indicating it's not a media file
                Err(StreamResultError {
                    filename: Some(filename),
                    reason: format!("Not a media file (detected as {media_type:?})"),
                    error_type: "not_media".to_string(),
                })
            }
        })
        .collect::<Result<Vec<_>, _>>()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![has_streams])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
