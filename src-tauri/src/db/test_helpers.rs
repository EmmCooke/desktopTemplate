//! Shared test utilities for database tests.

use sqlx::{Pool, Sqlite, SqlitePool};

/// Creates a fresh in-memory SQLite database with all migrations applied.
///
/// Each call returns an isolated database — tests can run in parallel
/// without interfering with each other.
pub async fn create_test_db() -> Pool<Sqlite> {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}
