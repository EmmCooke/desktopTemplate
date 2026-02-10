//! SQLite connection pool and migration management.

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;
use tauri::{AppHandle, Manager};

/// Creates a connection pool and runs all pending migrations.
///
/// The database file is stored in the platform-appropriate app data directory.
/// It is created automatically on first launch.
///
/// # Errors
///
/// Returns an error if the app data directory cannot be resolved, the database
/// connection fails, or migrations fail to apply.
pub async fn initialize(app: &AppHandle) -> Result<Pool<Sqlite>, Box<dyn std::error::Error>> {
    let app_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_dir)?;

    let db_path = app_dir.join("app.db");
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());

    let options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
