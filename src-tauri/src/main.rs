#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use sha2::{Digest, Sha256};
use tauri::{Manager, RunEvent};
use webkit2gtk::{PermissionRequestExt, WebViewExt};
use webkit2gtk_sys::request;

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![called_from_js, hash256sum,])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.with_webview(|webview| {
                webview.inner().connect_permission_request(|_, request| {
                    println!("{}", request.);
                    request.allow();
                    true
                });
            });
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_, e| match e {
        RunEvent::Ready => {
            println!("Window is ready");
        }
        _ => {}
    })
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
