mod db;
mod config;
mod routes;

use actix_web::{App, HttpServer, middleware::Logger, web };
use actix_cors::Cors;
use sea_orm::Database;
use config::Config;
use std::sync::Arc;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();

    let db = db::init(&config).await;

    let app_state = web::Data::new(AppState {
        config: Arc::new(config),
        db,
    });

    println!(
        "🚀 Server running at http://{}:{}",
        app_state.config.host, app_state.config.port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive()) // TODO: lock this down later
            .app_data(app_state.clone())
            .configure(routes::init)
    })
    .bind((app_state.config.host.as_str(), app_state.config.port))?
    .run()
    .await
}

pub struct AppState {
    pub config: Arc<Config>,
    pub db: sea_orm::DatabaseConnection,
}