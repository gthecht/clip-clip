use geo::{GeodesicArea, MultiPolygon};
use geo_clipper::Clipper;
use geojson;

use crate::types::{CoverResponse, PartialCoverage};

/// Given some input subject (either polygon or multipolygon), and a series of other clippers
/// Return the leftover of the subject after intersecting with all the clippers one after the other
pub fn calculate_leftover(subject: &MultiPolygon, clippers: &Vec<MultiPolygon>) -> MultiPolygon {
    let clipped: MultiPolygon = clippers
        .iter()
        .fold(
            None,
            |leftover: Option<MultiPolygon>, clip| match leftover {
                Some(current_leftover) => Some(current_leftover.difference(clip, 1.0)),
                None => Some(subject.difference(clip, 1.0)),
            },
        )
        .unwrap();
    clipped
}

pub fn get_partial_coverage(subject: MultiPolygon, clippers: Vec<MultiPolygon>) -> PartialCoverage {
    let leftover: MultiPolygon = calculate_leftover(&subject, &clippers);
    let leftover_geojson = geojson::Geometry::from(&leftover);
    let intersection: MultiPolygon = calculate_leftover(&subject, &vec![leftover.clone()]);
    let intersection_geojson = geojson::Geometry::from(&intersection);
    let covered_percentage =
        100.0 * (intersection.geodesic_area_unsigned() / subject.geodesic_area_unsigned());

    PartialCoverage::new(
        covered_percentage,
        Some(leftover_geojson),
        Some(intersection_geojson),
    )
}

pub fn get_coverage(subject: MultiPolygon, clippers: Vec<MultiPolygon>) -> CoverResponse {
    let full_coverage = get_partial_coverage(subject.clone(), clippers.clone());
    let partial_coverages: Vec<PartialCoverage> = clippers
        .into_iter()
        .map(|clip| get_partial_coverage(subject.clone(), vec![clip]))
        .collect();
    CoverResponse::new(
        full_coverage.covered_percentage,
        full_coverage.leftover,
        full_coverage.covered_area,
        Some(partial_coverages),
    )
}

#[cfg(test)]
mod leftover_test {
    use super::*;
    use eyre::Result;
    use geo::{Coord, LineString, MultiPolygon, Polygon};

    #[test]
    fn calculate_leftover_test() -> Result<()> {
        let subject = MultiPolygon(vec![Polygon::new(
            LineString(vec![
                Coord { x: 40.0, y: 50.0 },
                Coord { x: 30.0, y: 50.0 },
                Coord { x: 30.0, y: 0.0 },
                Coord { x: 40.0, y: 0.0 },
            ]),
            vec![],
        )]);

        let clip1 = MultiPolygon::new(vec![Polygon::new(
            LineString(vec![
                Coord { x: 40.0, y: 50.0 },
                Coord { x: 30.0, y: 50.0 },
                Coord { x: 30.0, y: 0.0 },
                Coord { x: 40.0, y: 0.0 },
            ]),
            vec![LineString(vec![
                Coord { x: 35.0, y: 35.0 },
                Coord { x: 30.0, y: 20.0 },
                Coord { x: 36.0, y: 30.0 },
            ])],
        )]);

        let clip2 = MultiPolygon::new(vec![Polygon::new(
            LineString(vec![
                Coord { x: 15.0, y: 15.0 },
                Coord { x: 10.0, y: 10.0 },
                Coord { x: 16.0, y: 10.0 },
            ]),
            vec![],
        )]);

        let result = calculate_leftover(&subject, &vec![clip1, clip2]);
        let expected = MultiPolygon::new(vec![Polygon::new(
            LineString(vec![
                Coord { x: 35.0, y: 35.0 },
                Coord { x: 30.0, y: 20.0 },
                Coord { x: 36.0, y: 30.0 },
            ]),
            vec![],
        )]);
        assert_eq!(expected, result);
        Ok(())
    }
}
