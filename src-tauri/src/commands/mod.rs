//! Tauri command handlers.
//!
//! Each file in this module exposes `#[tauri::command]` functions that are
//! registered in [`crate::run`]. Keep commands thin — delegate business
//! logic to the `db` module or standalone functions.

pub mod greet;
