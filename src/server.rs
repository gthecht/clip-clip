use crate::{
    leftover::get_coverage,
    types::{CoverResponse, GetCoverBody},
};
use axum::{
    routing::{get, post},
    Json, Router,
};

async fn get_coverage_handler(Json(payload): Json<GetCoverBody>) -> Json<CoverResponse> {
    let response = get_coverage(payload.subject, payload.clippers);
    Json(response)
}

async fn get_health() -> String {
    "GOOD".to_string()
}

pub fn create_router() -> Router {
    Router::new()
        .route("/coverage", post(get_coverage_handler))
        .route("/health", get(get_health))
}
