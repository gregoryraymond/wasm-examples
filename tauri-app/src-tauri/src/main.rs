// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet() -> String {
    let worker_response = match reqwest::get("http://localhost:8787/name").await {
        Ok(x) => {
            match x.text().await {
                 Ok(y) => {
                    y
                },
                Err(e) => { String::from(format!("Error: {}", e)) }
            }
        },
        Err(e) => { String::from(format!("Error: {}", e)) }
    };
    worker_response
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
