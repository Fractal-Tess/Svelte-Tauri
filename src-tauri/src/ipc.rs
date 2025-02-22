#![allow(clippy::needless_pass_by_value)]
// This module shows examples of how to use IPC command handlers that can be invoked from the frontend.
use crate::state::Store;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use specta::Type;
use specta_typescript::Typescript;
use tauri::{Builder, State, Wry};
use tauri_specta::{collect_commands, collect_events, Builder as SpectaBuilder, Event};

// Function that mounts all commands and events to the app instance and generates the typescript bindings.
pub fn register_ipc_handlers(app: Builder<Wry>) -> Builder<Wry> {
    // Create a specta builder;
    let builder = SpectaBuilder::<tauri::Wry>::new()
        // Register all commands
        .commands(collect_commands![
            hello_tauri,
            hash256sum,
            store_set_key,
            store_read_key
        ])
        // Register all events
        .events(collect_events![ExampleEvent]);

    // Generate the IPC types
    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/lib/ipc.ts")
        .expect("Failed to export typescript bindings");

    // Register the commands and events in the tauri app instance
    let app = app.invoke_handler(builder.invoke_handler());

    // Mount the events
    app.invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);

            // Now you can use them
            ExampleEvent::listen(app, |event| {
                println!("{:?}", event.payload);
            });

            ExampleEvent("Test".into()).emit(app).unwrap();

            Ok(())
        })
}

// An example event
#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct ExampleEvent(String);

// An example command
#[tauri::command]
#[specta::specta]
fn hello_tauri() -> String {
    "Hi from Tauri".to_owned()
}

// Another one
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

// Another one
#[tauri::command]
#[specta::specta]
fn store_read_key(key: String, store: State<Store>) -> Option<String> {
    store.read_key(&key)
}
