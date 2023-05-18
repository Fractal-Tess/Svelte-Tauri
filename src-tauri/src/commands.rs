use sha2::{Digest, Sha256};
use specta::collect_types;
use tauri::{Builder, Wry};
use tauri_specta::ts;

// Exports a function for the tauri app instance to use and register all commands defined as frontend IPC command handlers.
pub fn register_commands(builder: Builder<Wry>) -> Builder<Wry> {
    // Specta generating typed binding interfaces
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![hello_tauri, hash256sum],
        "../src/lib/bindings.ts",
    )
    .expect("unable to generate specta types");

    builder.invoke_handler(tauri::generate_handler![hash256sum, hello_tauri])
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
    format!("{:X}", result)
}
