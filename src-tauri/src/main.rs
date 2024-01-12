// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod split_video;
mod parser;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(name: String) -> Result<String, String> {
    // sleep 10 seconds to simulate a long running task
    std::thread::sleep(std::time::Duration::from_secs(10));
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
    // format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn split_file(file_names: Vec<String>, time_stamp: String) -> Result<String, String> {
    let result = split_video::split_file(file_names, time_stamp);
    match result {
        Ok(_) => {},
        Err(e) => {
            return Err(e.to_string());
        }
    }
    Ok("Split file success!".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,split_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
