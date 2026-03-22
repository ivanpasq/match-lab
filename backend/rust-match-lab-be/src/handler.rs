use tracing::info;

use crate::config::config;

pub async fn healthz_handler() {
    "OK";
}

pub async fn leagues_handler() {
    info!("Starting data retrieve from SoFIFA...");

    let client = reqwest::Client::new();

    let response = client.get("https://api.sofifa.net/leagues")
        .bearer_auth(config().sofifa_api_token.clone())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    info!(response);
}