// This module shows examples of how to use managed custom state.

use std::collections::HashMap;
use std::string::ToString;
use std::sync::Mutex;

use tauri::{Builder, Wry};

// Exports a function for the tauri app instance to use and register all commands defined as frontend IPC command handlers.
pub fn register_managed_state(builder: Builder<Wry>) -> Builder<Wry> {
    let store = Store::default();

    builder.manage(store)
}

#[derive(Default)]
pub struct Store {
    store: Mutex<HashMap<String, String>>,
}
impl Store {
    pub fn add_key_val(&self, key: String, val: String) {
        self.store
            .lock()
            .expect("cannot lock store")
            .insert(key, val);
    }
    pub fn read_key(&self, key: &String) -> Option<String> {
        self.store
            .lock()
            .expect("cannot lock store")
            .get(key)
            .map(ToString::to_string)
    }
}
