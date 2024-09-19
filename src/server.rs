use crate::types::{CoverResponse, GetCoverBody};
use axum::{routing::post, Json, Router};

async fn get_coverage(Json(_payload): Json<GetCoverBody>) -> Json<CoverResponse> {
    let response = CoverResponse::new(0.0, None, None, None);
    Json(response)
}

pub fn create_router() -> Router {
    Router::new().route("/coverage", post(get_coverage))
}
