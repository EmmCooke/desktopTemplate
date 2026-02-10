//! Unified error types for the application.

use serde::Serialize;
use thiserror::Error;

/// Application-wide error type that can cross the Tauri IPC boundary.
#[derive(Debug, Error)]
pub enum AppError {
    /// A database operation failed.
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// An IO operation failed.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// The requested resource was not found.
    #[error("Not found: {0}")]
    NotFound(String),

    /// Input validation failed.
    #[error("Validation error: {0}")]
    Validation(String),
}

// Tauri requires Serialize to send errors across IPC.
// We serialize as a string so the frontend gets a human-readable message.
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

/// Convenient type alias for functions returning [`AppError`].
pub type AppResult<T> = Result<T, AppError>;
