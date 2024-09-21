use geojson::Geometry;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Deserialize)]
pub struct GeoArea {
    #[serde(rename = "_id")]
    id: Option<String>,
    pub area: Geometry, // This isn't a geometry but either a polygon or a multi-polygon
}

#[derive(Debug, Deserialize)]
pub struct GetCoverBody {
    #[serde(rename = "areaToBeCovered")]
    pub subject: GeoArea,
    #[serde(rename = "intersectingCandidates")]
    pub clippers: Vec<GeoArea>,
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
