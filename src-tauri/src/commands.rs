#![allow(clippy::needless_pass_by_value)]
// This module shows examples of how to use IPC command handlers that can be invoked from the frontend.
use sha2::{Digest, Sha256};
use specta::collect_types;
use tauri::{Builder, State, Wry};
use tauri_specta::ts;

use crate::state::Store;

// Exports a function for the tauri app instance to use and register all commands defined as frontend IPC command handlers.
pub fn register_command_handlers(builder: Builder<Wry>) -> Builder<Wry> {
    // Specta generating typed binding interfaces
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![hello_tauri, hash256sum, store_set_key, store_read_key],
        "../src/lib/ipc.ts",
    )
    .expect("unable to generate specta types");

    builder.invoke_handler(tauri::generate_handler![
        hash256sum,
        hello_tauri,
        store_set_key,
        store_read_key
    ])
}

// An example command
#[tauri::command]
#[specta::specta]
fn hello_tauri() -> String {
    "Hi from Tauri".to_owned()
}

// Another command
#[tauri::command]
#[specta::specta]
fn hash256sum(hash_input: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(hash_input.as_bytes());
    let result = hasher.finalize();
    format!("{result:X}")
}

// Example command using managed state
#[tauri::command]
#[specta::specta]
fn store_set_key(key: String, value: String, store: State<Store>) {
    store.add_key_val(key, value);
}

// Another example command using managed state
#[tauri::command]
#[specta::specta]
fn store_read_key(key: String, store: State<Store>) -> Option<String> {
    store.read_key(&key)
}
