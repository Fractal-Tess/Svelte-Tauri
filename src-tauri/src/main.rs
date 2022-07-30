#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use sha2::{Digest, Sha256};
use tauri_plugin_store::PluginBuilder;
// use tauri::{Manager, PhysicalSize, Size};

fn main() {
    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![called_from_js, hash256sum])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn called_from_js() -> String {
    // The print macro is problematic in release environment (crashes the application if not ran from a terminal)
    // println!("Returning from tauri");
    "Hi from Tauri".to_owned()
}

#[tauri::command]
fn hash256sum(hash_input: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(hash_input.as_bytes());
    let result = hasher.finalize();
    format!("{:X}", result)
}
