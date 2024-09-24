use crate::{
    leftover::get_coverage,
    types::{CoverResponse, GetCoverBody},
};
use axum::{
    routing::{get, post},
    Json, Router,
};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

async fn get_coverage_handler(Json(payload): Json<GetCoverBody>) -> Json<CoverResponse> {
    let response = get_coverage(payload.subject, payload.clippers);
    Json(response)
}

async fn get_health() -> String {
    "GOOD".to_string()
}

pub fn create_router() -> Router {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    Router::new()
        .route("/coverage", post(get_coverage_handler))
        .route("/health", get(get_health))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new()
                    .level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new()
                    .level(Level::INFO)),
        )
}
