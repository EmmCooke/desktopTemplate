//! Database models.
//!
//! Each struct here maps to a SQLite table row. Derive both `serde::Serialize`
//! (for sending across IPC) and `sqlx::FromRow` (for query results).

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Example model for the `items` table.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Item {
    /// Auto-incrementing primary key.
    pub id: i64,
    /// Display name of the item.
    pub title: String,
    /// Optional description.
    pub description: Option<String>,
    /// ISO 8601 timestamp of creation.
    pub created_at: String,
}
