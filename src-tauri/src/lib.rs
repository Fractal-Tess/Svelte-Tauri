use tauri::Builder as TauriBuilder;

mod error;
mod ipc;
mod prelude;
mod state;

use crate::ipc::register_ipc_handlers;
use crate::state::register_managed_state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Application
    let app = TauriBuilder::default();

    // Plugins
    let app = app.plugin(tauri_plugin_opener::init());

    // Commands
    let app = register_ipc_handlers(app);

    // State
    let app = register_managed_state(app);

    // Run
    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}
