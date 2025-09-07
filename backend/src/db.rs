use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn get_pool() -> SqlitePool {
    SqlitePoolOptions::new()
        .connect("sqlite:clinic.db")
        .await.expect("Failed to connect to SQLite")
}