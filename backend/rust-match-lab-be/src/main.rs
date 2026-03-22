mod config;
mod router;
mod handler;

use tracing::{Level, info};
use std::str::FromStr;

use crate::{config::config, router::build_router};

#[tokio::main]
async fn main() {
    info!("Starting...");

    config();

    let level = Level::from_str(&config().log_level).unwrap_or(Level::INFO);
    tracing_subscriber::fmt().with_max_level(level).init();

    let router = build_router();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config().http_port)).await.unwrap();

    info!("Listener ready on port {}", config().http_port);

    axum::serve(listener, router).await.unwrap();
}