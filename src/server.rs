use crate::{
    leftover::get_coverage,
    types::{CoverResponse, GetCoverBody},
};
use axum::{routing::post, Json, Router};

async fn get_coverage_handler(Json(payload): Json<GetCoverBody>) -> Json<CoverResponse> {
    let response = get_coverage(payload.subject, payload.clippers);
    Json(response)
}

pub fn create_router() -> Router {
    Router::new().route("/coverage", post(get_coverage_handler))
}
