

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tauri::Manager;
use window_shadows::set_shadow;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]

fn main() {
        tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]{
                let window = app.get_window("main").unwrap();
                window.open_devtools(); 
                set_shadow(&window, true).expect("Unsupported platform!");
            }
            Ok(())
        })
.invoke_handler(tauri::generate_handler![])
.run(tauri::generate_context!())
.expect("error while running tauri application");
}
