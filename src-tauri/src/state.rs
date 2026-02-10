//! Application state managed by Tauri.

use sqlx::{Pool, Sqlite};

/// Shared application state accessible in Tauri commands via `State<'_, AppState>`.
///
/// Tauri's `manage()` wraps this in an `Arc` automatically — do not add your own.
pub struct AppState {
    /// SQLite connection pool.
    pub db: Pool<Sqlite>,
}
