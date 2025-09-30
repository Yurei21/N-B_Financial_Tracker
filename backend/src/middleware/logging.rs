use tracing_subscriber::{FmtSubscriber, EnvFilter};
use actix_web::middleware::Logger;

pub fn init_tracing() {
    let env = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new(env))
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber");
}

pub fn actix_logger() -> Logger {
    Logger::default()
}