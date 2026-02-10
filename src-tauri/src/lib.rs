//! Desktop Template — Tauri application backend.
//!
//! This crate provides the Rust backend for the desktop application.
//! All database access, business logic, and Tauri commands live here.

#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod commands;
pub mod db;
pub mod error;
pub mod state;

use state::AppState;
use tauri::Manager;

/// Builds and runs the Tauri application.
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let db_pool = tauri::async_runtime::block_on(db::connection::initialize(app.handle()))?;
            app.manage(AppState { db: db_pool });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::greet::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
