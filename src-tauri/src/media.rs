use crate::models::MediaType;
use std::{fs::File, io::Read, path::Path};
use tracing::{debug, instrument, trace, warn};

#[instrument(skip(path), fields(path = %path.display()))]
pub fn detect_media_type(path: &Path) -> MediaType {
    debug!("Starting media type detection");

    // First try to detect using infer crate (magic number detection)
    if let Ok(mut file) = File::open(path) {
        let mut buffer = [0; 512];
        if let Ok(bytes_read) = file.read(&mut buffer) {
            trace!(bytes_read = bytes_read, "Read file header for magic number detection");
            
            if let Some(kind) = infer::get(&buffer[..bytes_read]) {
                let mime_type = kind.mime_type();
                debug!(mime_type = %mime_type, "Detected MIME type from magic numbers");
                
                let media_type = match mime_type {
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
                
                debug!(media_type = ?media_type, "Detected media type from magic numbers");
                return media_type;
            } else {
                debug!("Magic number detection failed, falling back to extension-based detection");
            }
        } else {
            warn!("Failed to read file for magic number detection");
        }
    } else {
        warn!("Failed to open file for magic number detection");
    }

    // Fallback to extension-based detection
    if let Some(extension) = path.extension() {
        let ext_str = extension.to_str().unwrap_or("").to_lowercase();
        debug!(extension = %ext_str, "Detecting media type from file extension");
        
        let media_type = match ext_str.as_str() {
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
            | "cpio" | "par2" | "epub" | "mobi" => MediaType::Archive,

            // Executable extensions
            "exe" | "dll" | "msi" | "dmg" | "pkg" | "app" | "elf" | "bc" | "mach" | "class"
            | "dex" | "dey" | "der" | "obj" => MediaType::Executable,

            // Library extensions
            "so" | "dylib" => MediaType::Library,

            _ => MediaType::Unknown,
        };
        
        debug!(media_type = ?media_type, "Detected media type from extension");
        media_type
    } else {
        debug!("No file extension found, returning Unknown");
        MediaType::Unknown
    }
}

#[instrument(skip(media_type))]
pub fn is_media_file(media_type: &MediaType) -> bool {
    let is_media = matches!(
        media_type,
        MediaType::Audio | MediaType::Video | MediaType::Image
    );
    
    debug!(media_type = ?media_type, is_media = is_media, "Checking if file is media type");
    is_media
}
