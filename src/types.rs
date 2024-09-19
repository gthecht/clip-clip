use geo::Geometry;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::leftover;

#[derive(Debug, Deserialize)]
pub struct GeoArea {
    #[serde(rename = "_id")]
    id: Option<String>,
    area: Geometry,
}

#[derive(Debug, Deserialize)]
pub struct GetCoverBody {
    #[serde(rename = "areaToBeCovered")]
    subject: GeoArea,
    #[serde(rename = "intersectingCandidates")]
    clippers: Vec<GeoArea>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct PartialCoverage {
    #[serde(rename = "covered%")]
    covered_percent: f64,
    leftover: Option<Geometry>,
    #[serde(rename = "coveredArea")]
    covered_area: Option<Geometry>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct CoverResponse {
    #[serde(rename = "covered%")]
    covered_percent: f64,
    leftover: Option<Geometry>,
    #[serde(rename = "coveredArea")]
    covered_area: Option<Geometry>,
    #[serde(rename = "partialCoverages")]
    partial_coverages: Option<Vec<PartialCoverage>>,
}

impl CoverResponse {
    pub fn new(
        covered_percent: f64,
        leftover: Option<Geometry>,
        covered_area: Option<Geometry>,
        partial_coverages: Option<Vec<PartialCoverage>>,
    ) -> Self {
        CoverResponse {
            covered_percent,
            leftover,
            covered_area,
            partial_coverages,
        }
    }
}
