use serde::{Deserialize, Serialize};
use specta::Type;
use specta_typescript::Typescript;
use std::path::Path;
use tauri_specta::{collect_commands, Builder};

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
struct StreamResult {
    path: String,
    filename: String,
    streams: Vec<StreamDetail>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
enum StreamDetail {
    Video { codec: String },
    Audio { codec: String },
    Subtitle { codec: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
struct StreamResultError {
    filename: Option<String>,
    reason: String,
}

#[tauri::command]
#[specta::specta]
fn has_streams(paths: Vec<String>) -> Result<Vec<StreamResult>, StreamResultError> {
    paths
        .into_iter()
        .map(|path_str| {
            let path = Path::new(&path_str);
            let filename = path.file_name().unwrap().to_str().unwrap().to_string();

            if !path.exists() {
                return Err(StreamResultError {
                    filename: Some(filename),
                    reason: "File does not exist".to_string(),
                });
            }
            if !path.is_file() {
                return Err(StreamResultError {
                    filename: Some(filename),
                    reason: "Not a file".to_string(),
                });
            }

            match ffprobe::ffprobe(&path_str) {
                Ok(info) => {
                    dbg!(info);
                    Ok(StreamResult {
                        filename,
                        path: path_str,
                        streams: vec![],
                    })
                }
                Err(err) => {
                    eprintln!("Could not analyze file with ffprobe: {:?}", err);
                    Err(StreamResultError {
                        filename: Some(filename),
                        reason: "Could not analyze file with ffprobe".to_string(),
                    })
                }
            }
        })
        .collect::<Result<Vec<_>, _>>()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![has_streams,]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![has_streams])
        .setup(move |app| {
            // Ensure you mount your events!
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
