mod config;
mod db;
mod errors;
mod routes;
mod middleware;
mod handlers;
mod services;
mod entities;

use actix_web::{App, HttpServer, web, middleware::Logger};
use actix_cors::Cors;
use dotenvy::dotenv;
use std::env;
use tracing_subscriber::FmtSubscriber;

use crate::config::Config;
use crate::db::connect;
use crate::routes::config as route_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize .env and logger
    dotenv().ok();

    // Setup tracer logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Fail to set logger");

    // Load configuration
    let config = Config::from_env().expect("Failed to load config");

    // 
    let db = connect(&config).await;
    tracing::info!("Connected to database");

    // Wrap in Actix `Data` for shared state
    let db_data = web::Data::new(db);
    let config_data = web::Data::new(config.clone());

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(db_data.clone())
            .app_data(config_data.clone())
            .configure(route_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}