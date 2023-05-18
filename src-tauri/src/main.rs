#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use commands::register_commands;
use state::register_managed_state;
use tauri::{Builder as TauriBuilder, RunEvent};

mod commands;
mod error;
mod prelude;
mod state;

fn main() {
    // App builder
    let app = TauriBuilder::default().plugin(tauri_plugin_window_state::Builder::default().build());

    // Register app commands
    let app = register_commands(app);

    // Register app managed state
    let app = register_managed_state(app);

    // Run the app
    app.build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_, e| match e {
            RunEvent::Ready => {
                println!("Window is ready");
            }
            _ => {}
        })
}
