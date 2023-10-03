#![allow(unused)] // While exploring, remove for prod.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub use error::{Error, Result};

use model::Store;
use std::sync::Arc;

mod error;
mod ipc;
mod macros;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    let store = Store::new().await?;
    let store = Arc::new(store);

    tauri::Builder::default()
        .manage(store)
        .invoke_handler(tauri::generate_handler![
            ipc::create_person,
            ipc::get_person,
            ipc::update_person,
            ipc::delete_person,
            ipc::list_persons,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
