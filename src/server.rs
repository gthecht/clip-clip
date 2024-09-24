use crate::{
    leftover::get_coverage,
    types::{CoverResponse, GetCoverBody},
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use eyre::Result;
use geo::MultiPolygon;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

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
    let subject: MultiPolygon = payload
        .subject
        .area
        .try_into()
        .expect("subject should be valid multi polygon or polygon");
    let clippers: Vec<MultiPolygon> = payload
        .clippers
        .into_iter()
        .map(|clip| {
            MultiPolygon::try_from(clip.area)
                .expect("clipper should be valid multi polygon or polygon")
        })
        .collect();
    let response = get_coverage(subject, clippers);
    Ok(Json(response))
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
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
