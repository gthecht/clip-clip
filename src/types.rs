use geojson::Geometry;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, Deserialize)]
pub struct GeoArea {
    pub area: Geometry,
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
    pub covered_percentage: f64,
    pub leftover: Option<Geometry>,
    #[serde(rename = "coveredArea")]
    pub covered_area: Option<Geometry>,
}

impl PartialCoverage {
    pub fn new(
        covered_percentage: f64,
        leftover: Option<Geometry>,
        covered_area: Option<Geometry>,
    ) -> Self {
        PartialCoverage {
            covered_percentage,
            leftover,
            covered_area,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct CoverResponse {
    #[serde(rename = "covered%")]
    covered_percentage: f64,
    leftover: Option<Geometry>,
    #[serde(rename = "coveredArea")]
    covered_area: Option<Geometry>,
    #[serde(rename = "partialCoverages")]
    partial_coverages: Option<Vec<PartialCoverage>>,
}

impl CoverResponse {
    pub fn new(
        covered_percentage: f64,
        leftover: Option<Geometry>,
        covered_area: Option<Geometry>,
        partial_coverages: Option<Vec<PartialCoverage>>,
    ) -> Self {
        CoverResponse {
            covered_percentage,
            leftover,
            covered_area,
            partial_coverages,
        }
    }
}
