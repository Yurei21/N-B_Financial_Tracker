use sea_orm::{Database, DatabaseConnection};
use actix_web::web::Data;
use crate::config::Config;

/// Connects directly to the database using the URL in config
pub async fn connect(config: &Config) -> DatabaseConnection {
    let db_url = &config.database_url;
    Database::connect(db_url)
        .await
        .unwrap_or_else(|e| panic!("❌ Failed to connect to database '{}': {}", db_url, e))
}

/// Convenience wrapper to return Actix-compatible `Data<DatabaseConnection>`
pub async fn connect_and_wrap(config: Data<Config>) -> Data<DatabaseConnection> {
    let conn = connect(&config).await;
    Data::new(conn)
}
