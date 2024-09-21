use crate::{
    leftover::get_coverage,
    types::{CoverResponse, GetCoverBody},
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use eyre::Result;

struct ErrorReport(eyre::Report);

impl IntoResponse for ErrorReport {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error occurred: {}", self.0),
        )
            .into_response()
    }
}

impl<R> From<R> for ErrorReport
where
    R: Into<eyre::Report>,
{
    fn from(report: R) -> Self {
        Self(report.into())
    }
}

async fn get_coverage_handler(
    Json(payload): Json<GetCoverBody>,
) -> Result<Json<CoverResponse>, ErrorReport> {
    let response = get_coverage(payload.subject, payload.clippers)?;
    Ok(Json(response))
}

pub fn create_router() -> Router {
    Router::new().route("/coverage", post(get_coverage_handler))
}
