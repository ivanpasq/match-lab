mod config;

use axum::{Router, routing::get};
use tracing::{Level, info};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    info!("Starting...");

    let config = config::Config::builder();

    let level = Level::from_str(&config.log_level).unwrap_or(Level::INFO);
    tracing_subscriber::fmt().with_max_level(level).init();

    let app = Router::new()
        .route("/healthz", get(|| async { "OK" }));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.http_port)).await.unwrap();

    info!("Listener ready on port {}", config.http_port);

    axum::serve(listener, app).await.unwrap();
}