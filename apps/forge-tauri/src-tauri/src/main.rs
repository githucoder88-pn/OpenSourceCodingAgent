#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
async fn get_forge_core_status() -> Result<String, String> {
    Ok("Forge Core running on http://localhost:3000".to_string())
}

#[tauri::command]
async fn start_forge_core() -> Result<String, String> {
    Ok("Forge Core start requested".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_forge_core_status, start_forge_core])
        .setup(|app| {
            println!("Forge Tauri app started");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
