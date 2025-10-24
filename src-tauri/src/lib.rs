pub mod ff;
pub mod media;
pub mod models;
pub mod strings;

use ff::extract_streams;
use media::{detect_media_type, is_media_file};
use models::{StreamResult, StreamResultError, File, FileCandidacy, BitrateData, BitrateFrame};
use strings::transform_filename;
use std::path::Path;
use std::process::Command;
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

#[tauri::command]
#[instrument(skip(paths), fields(file_count = paths.len()))]
fn analyze_files(paths: Vec<String>) -> Vec<File> {
    info!(file_count = paths.len(), "Analyzing files for candidacy");
    
    paths
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

            // Get file size
            let size = std::fs::metadata(&path_str)
                .map(|metadata| metadata.len())
                .unwrap_or(0) as u32;

            let truncated_name = transform_filename(&filename, 15);
            debug!(filename = %truncated_name, size = size, "File metadata retrieved");

            // Check if file exists
            if !path.exists() {
                let truncated_name = transform_filename(&filename, 15);
                warn!(filename = %truncated_name, "File does not exist");
                return File {
                    filename,
                    size,
                    candidacy: FileCandidacy::Error {
                        reason: "File does not exist".to_string(),
                    },
                };
            }

            // Check if it's a file (not directory)
            if !path.is_file() {
                let truncated_name = transform_filename(&filename, 15);
                warn!(filename = %truncated_name, "Path is not a file");
                return File {
                    filename,
                    size,
                    candidacy: FileCandidacy::Error {
                        reason: "Not a file (directory or other)".to_string(),
                    },
                };
            }

            // Detect media type using magic numbers and fallback to extensions
            let media_type = detect_media_type(path);
            debug!(filename = %truncated_name, media_type = ?media_type, "Media type detected");

            // Check if it's a media file
            if is_media_file(&media_type) {
                info!(filename = %truncated_name, media_type = ?media_type, "Valid media file detected");
                File {
                    filename,
                    size,
                    candidacy: FileCandidacy::Success {
                        file_type: media_type,
                    },
                }
            } else {
                debug!(filename = %truncated_name, media_type = ?media_type, "Non-media file detected");
                File {
                    filename,
                    size,
                    candidacy: FileCandidacy::Error {
                        reason: format!("Not a media file (detected as {media_type:?})"),
                    },
                }
            }
        })
        .collect()
}

#[tauri::command]
#[instrument(skip(path), fields(path = %path))]
fn extract_bitrate_data(path: String) -> Result<BitrateData, String> {
    info!(path = %path, "Extracting bitrate data from video file");

    let path_obj = Path::new(&path);
    let filename = path_obj
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Check if file exists
    if !path_obj.exists() {
        error!(filename = %filename, "File does not exist");
        return Err("File does not exist".to_string());
    }

    // Run ffprobe to get frame packet sizes
    // -v quiet: suppress ffprobe info
    // -select_streams v:0: only first video stream
    // -show_entries frame=pkt_size: only show packet size
    // -of csv=p=0: output as CSV without headers
    info!(filename = %filename, "Running ffprobe to extract frame data");

    let output = Command::new("ffprobe")
        .args([
            "-v", "quiet",
            "-select_streams", "v:0",
            "-show_entries", "frame=pkt_size",
            "-of", "csv=p=0",
            &path
        ])
        .output()
        .map_err(|e| {
            error!(error = %e, "Failed to execute ffprobe");
            format!("Failed to execute ffprobe: {e}")
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!(stderr = %stderr, "ffprobe command failed");
        return Err(format!("ffprobe failed: {stderr}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    debug!(line_count = stdout.lines().count(), "Parsing ffprobe output");

    let frames: Vec<BitrateFrame> = stdout
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            line.trim().parse::<u64>().ok().map(|packet_size| BitrateFrame {
                frame_num: index as u32,
                packet_size,
            })
        })
        .collect();

    if frames.is_empty() {
        warn!(filename = %filename, "No frame data extracted");
        return Err("No frame data could be extracted from file".to_string());
    }

    info!(
        filename = %filename,
        frame_count = frames.len(),
        "Successfully extracted bitrate data"
    );

    Ok(BitrateData {
        id: filename,
        frames,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    info!("Initializing Tauri application");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![has_streams, analyze_files, extract_bitrate_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}