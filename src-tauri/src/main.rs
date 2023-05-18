#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use commands::register_commands;
use prelude::*;

use tauri::RunEvent;
mod commands;
mod error;
mod prelude;

fn main() {
    let app =
        tauri::Builder::default().plugin(tauri_plugin_window_state::Builder::default().build());

    let app = register_commands(app)
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_, e| match e {
        RunEvent::Ready => {
            println!("Window is ready");
        }
        _ => {}
    })
}
