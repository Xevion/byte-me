use std::path::Path;

#[tauri::command]
fn has_streams(paths: Vec<String>) -> Result<Vec<bool>, String> {
    let mut results = Vec::with_capacity(paths.len());
    for path_str in paths {
        let path = Path::new(&path_str);
        if !path.is_file() {
            results.push(false);
            continue;
        }

        match ffprobe::ffprobe(&path_str) {
            Ok(info) => {
                dbg!(info);
                results.push(true);
            },
            Err(err) => {
                eprintln!("Could not analyze file with ffprobe: {:?}", err);
                results.push(false);
            }
        }
    }
    Ok(results)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![has_streams])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
