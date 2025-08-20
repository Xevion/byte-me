use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
enum MediaType {
    Audio,
    Video,
    Image,
    Document,
    Executable,
    Archive,
    Library,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
struct StreamResult {
    path: String,
    filename: String,
    media_type: MediaType,
    duration: Option<f64>,
    size: u64,
    streams: Vec<StreamDetail>,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
enum StreamDetail {
    Video {
        codec: String,
        width: Option<u32>,
        height: Option<u32>,
        bit_rate: Option<String>,
        frame_rate: Option<String>,
    },
    Audio {
        codec: String,
        sample_rate: Option<String>,
        channels: Option<u32>,
        bit_rate: Option<String>,
    },
    Subtitle {
        codec: String,
        language: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
struct StreamResultError {
    filename: Option<String>,
    reason: String,
    error_type: String,
}

fn detect_media_type(path: &Path) -> MediaType {
    // First try to detect using infer crate (magic number detection)
    if let Ok(mut file) = File::open(path) {
        let mut buffer = [0; 512]; // Read first 512 bytes for magic number detection
        if let Ok(bytes_read) = file.read(&mut buffer) {
            if let Some(kind) = infer::get(&buffer[..bytes_read]) {
                return match kind.mime_type() {
                    // Audio types
                    "audio/mpeg" | "audio/mp3" | "audio/m4a" | "audio/ogg" | "audio/x-flac"
                    | "audio/x-wav" | "audio/amr" | "audio/aac" | "audio/x-aiff"
                    | "audio/x-dsf" | "audio/x-ape" | "audio/midi" => MediaType::Audio,

                    // Video types
                    "video/mp4" | "video/x-m4v" | "video/x-matroska" | "video/webm"
                    | "video/quicktime" | "video/x-msvideo" | "video/x-ms-wmv" | "video/mpeg"
                    | "video/x-flv" => MediaType::Video,

                    // Image types
                    "image/jpeg"
                    | "image/png"
                    | "image/gif"
                    | "image/webp"
                    | "image/x-canon-cr2"
                    | "image/tiff"
                    | "image/bmp"
                    | "image/heif"
                    | "image/avif"
                    | "image/vnd.ms-photo"
                    | "image/vnd.adobe.photoshop"
                    | "image/vnd.microsoft.icon"
                    | "image/openraster"
                    | "image/vnd.djvu" => MediaType::Image,

                    // Document types
                    "application/pdf"
                    | "application/rtf"
                    | "application/msword"
                    | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
                    | "application/vnd.ms-excel"
                    | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                    | "application/vnd.ms-powerpoint"
                    | "application/vnd.openxmlformats-officedocument.presentationml.presentation"
                    | "application/vnd.oasis.opendocument.text"
                    | "application/vnd.oasis.opendocument.spreadsheet"
                    | "application/vnd.oasis.opendocument.presentation" => MediaType::Document,

                    // Archive types
                    "application/zip"
                    | "application/x-tar"
                    | "application/vnd.rar"
                    | "application/gzip"
                    | "application/x-bzip2"
                    | "application/vnd.bzip3"
                    | "application/x-7z-compressed"
                    | "application/x-xz"
                    | "application/x-shockwave-flash"
                    | "application/octet-stream"
                    | "application/postscript"
                    | "application/vnd.sqlite3"
                    | "application/x-nintendo-nes-rom"
                    | "application/x-google-chrome-extension"
                    | "application/vnd.ms-cab-compressed"
                    | "application/vnd.debian.binary-package"
                    | "application/x-unix-archive"
                    | "application/x-compress"
                    | "application/x-lzip"
                    | "application/x-rpm"
                    | "application/dicom"
                    | "application/zstd"
                    | "application/x-lz4"
                    | "application/x-ole-storage"
                    | "application/x-cpio"
                    | "application/x-par2"
                    | "application/epub+zip"
                    | "application/x-mobipocket-ebook" => MediaType::Archive,

                    // Executable types
                    "application/vnd.microsoft.portable-executable"
                    | "application/x-executable"
                    | "application/llvm"
                    | "application/x-mach-binary"
                    | "application/java"
                    | "application/vnd.android.dex"
                    | "application/vnd.android.dey"
                    | "application/x-x509-ca-cert" => MediaType::Executable,

                    // Library types (covered by executable types above, but keeping for clarity)
                    _ => MediaType::Unknown,
                };
            }
        }
    }

    // Fallback to extension-based detection
    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap_or("").to_lowercase().as_str() {
            // Audio extensions
            "mp3" | "wav" | "flac" | "ogg" | "m4a" | "aac" | "wma" | "mid" | "amr" | "aiff"
            | "dsf" | "ape" => MediaType::Audio,

            // Video extensions
            "mp4" | "mkv" | "webm" | "mov" | "avi" | "wmv" | "mpg" | "flv" | "m4v" => {
                MediaType::Video
            }

            // Image extensions
            "gif" | "png" | "jpg" | "jpeg" | "bmp" | "tiff" | "webp" | "cr2" | "heif" | "avif"
            | "jxr" | "psd" | "ico" | "ora" | "djvu" => MediaType::Image,

            // Document extensions
            "txt" | "md" | "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "odt"
            | "ods" | "odp" | "rtf" => MediaType::Document,

            // Archive extensions
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "bz3" | "xz" | "swf" | "sqlite"
            | "nes" | "crx" | "cab" | "deb" | "ar" | "Z" | "lz" | "rpm" | "dcm" | "zst" | "lz4"
            | "msi" | "cpio" | "par2" | "epub" | "mobi" => MediaType::Archive,

            // Executable extensions
            "exe" | "dll" | "msi" | "dmg" | "pkg" | "deb" | "rpm" | "app" | "elf" | "bc"
            | "mach" | "class" | "dex" | "dey" | "der" | "obj" => MediaType::Executable,

            // Library extensions
            "so" | "dylib" => MediaType::Library,

            _ => MediaType::Unknown,
        }
    } else {
        MediaType::Unknown
    }
}

fn is_media_file(media_type: &MediaType) -> bool {
    matches!(
        media_type,
        MediaType::Audio | MediaType::Video | MediaType::Image
    )
}

fn extract_streams(info: &ffprobe::FfProbe) -> Vec<StreamDetail> {
    let mut streams = Vec::new();

    for stream in &info.streams {
        match stream.codec_type.as_deref() {
            Some("video") => {
                streams.push(StreamDetail::Video {
                    codec: stream
                        .codec_name
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string()),
                    width: stream.width.map(|w| w as u32),
                    height: stream.height.map(|h| h as u32),
                    bit_rate: stream.bit_rate.as_ref().map(|b| b.to_string()),
                    frame_rate: Some(stream.r_frame_rate.clone()),
                });
            }
            Some("audio") => {
                streams.push(StreamDetail::Audio {
                    codec: stream
                        .codec_name
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string()),
                    sample_rate: stream.sample_rate.clone(),
                    channels: stream.channels.map(|c| c as u32),
                    bit_rate: stream.bit_rate.as_ref().map(|b| b.to_string()),
                });
            }
            Some("subtitle") => {
                streams.push(StreamDetail::Subtitle {
                    codec: stream
                        .codec_name
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string()),
                    language: stream.tags.as_ref().and_then(|tags| tags.language.clone()),
                });
            }
            _ => {}
        }
    }

    streams
}

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
                        eprintln!("Could not analyze media file with ffprobe: {:?}", err);
                        Err(StreamResultError {
                            filename: Some(filename),
                            reason: format!("Could not analyze media file: {}", err),
                            error_type: "analysis_failed".to_string(),
                        })
                    }
                }
            } else {
                // For non-media files, return an error indicating it's not a media file
                Err(StreamResultError {
                    filename: Some(filename),
                    reason: format!("Not a media file (detected as {:?})", media_type),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_bindings() {
        // This will generate TypeScript bindings when you run `cargo test export_bindings`
        StreamResult::export().unwrap();
        StreamDetail::export().unwrap();
        StreamResultError::export().unwrap();
    }
}
