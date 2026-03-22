use axum::{Router, routing::get};

use crate::handler::{healthz_handler, leagues_handler};

pub fn build_router() -> Router{
    Router::new()
        .route("/healthz", get(healthz_handler))
        .route("/api/leagues", get(leagues_handler))
}