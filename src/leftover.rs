use eyre::Result;
use geo::{GeodesicArea, MultiPolygon};
use geo_clipper::Clipper;
use geojson;

use crate::types::{CoverResponse, GeoArea, PartialCoverage};

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

pub fn get_partial_coverage(subject: GeoArea, clippers: Vec<GeoArea>) -> Result<PartialCoverage> {
    let subject: MultiPolygon = subject
        .area
        .try_into()
        .expect("expected subject to be valid geojson geometry");
    let clippers: Vec<MultiPolygon> = clippers
        .into_iter()
        .map(|clip| {
            MultiPolygon::try_from(clip.area)
                .expect("expected clipper to be valid geojson geometry")
        })
        .collect();
    let leftover: MultiPolygon = calculate_leftover(&subject, &clippers);
    let leftover_geojson = geojson::Geometry::from(&leftover);
    let intersection: MultiPolygon = calculate_leftover(&subject, &vec![leftover.clone()]);
    let intersection_geojson = geojson::Geometry::from(&intersection);
    let covered_percentage =
        100.0 * (intersection.geodesic_area_unsigned() / subject.geodesic_area_unsigned());

    Ok(PartialCoverage::new(
        covered_percentage,
        Some(leftover_geojson),
        Some(intersection_geojson),
    ))
}

pub fn get_coverage(subject: GeoArea, clippers: Vec<GeoArea>) -> Result<CoverResponse> {
    let full_coverage = get_partial_coverage(subject.clone(), clippers.clone())?;
    let partial_coverages: Result<Vec<PartialCoverage>> = clippers
        .into_iter()
        .map(|clip| get_partial_coverage(subject.clone(), vec![clip]))
        .collect();
    let response = CoverResponse::new(
        full_coverage.covered_percentage,
        full_coverage.leftover,
        full_coverage.covered_area,
        Some(partial_coverages?),
    );
    Ok(response)
}

#[cfg(test)]
mod leftover_test {
    use super::*;
    use eyre::Result;
    use geo::{Coord, Geometry, LineString, MultiPolygon, Polygon};

    #[test]
    fn polygon_leftover_test() -> Result<()> {
        let subject = Geometry::Polygon(Polygon::new(
            LineString(vec![
                Coord { x: 180.0, y: 200.0 },
                Coord { x: 260.0, y: 200.0 },
                Coord { x: 260.0, y: 150.0 },
                Coord { x: 180.0, y: 150.0 },
            ]),
            vec![],
        ));

        let clip1 = Polygon::new(
            LineString(vec![
                Coord { x: 190.0, y: 210.0 },
                Coord { x: 240.0, y: 210.0 },
                Coord { x: 240.0, y: 130.0 },
                Coord { x: 190.0, y: 130.0 },
            ]),
            vec![LineString(vec![
                Coord { x: 215.0, y: 160.0 },
                Coord { x: 230.0, y: 190.0 },
                Coord { x: 200.0, y: 190.0 },
            ])],
        );

        let clip2 = Polygon::new(
            LineString(vec![
                Coord { x: 215.0, y: 160.0 },
                Coord { x: 230.0, y: 190.0 },
                Coord { x: 200.0, y: 190.0 },
            ]),
            vec![],
        );

        let result = calculate_leftover(&subject, &vec![clip1, clip2])?;
        let expected = MultiPolygon::new(vec![
            Polygon::new(
                LineString(vec![
                    Coord { x: 190.0, y: 200.0 },
                    Coord { x: 180.0, y: 200.0 },
                    Coord { x: 180.0, y: 150.0 },
                    Coord { x: 190.0, y: 150.0 },
                    Coord { x: 190.0, y: 200.0 },
                ]),
                vec![],
            ),
            Polygon::new(
                LineString(vec![
                    Coord { x: 260.0, y: 200.0 },
                    Coord { x: 240.0, y: 200.0 },
                    Coord { x: 240.0, y: 150.0 },
                    Coord { x: 260.0, y: 150.0 },
                    Coord { x: 260.0, y: 200.0 },
                ]),
                vec![],
            ),
        ]);
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn multi_polygon_leftover_test() -> Result<()> {
        let subject = MultiPolygon(vec![Polygon::new(
            LineString(vec![
                Coord { x: 180.0, y: 200.0 },
                Coord { x: 260.0, y: 200.0 },
                Coord { x: 260.0, y: 150.0 },
                Coord { x: 180.0, y: 150.0 },
            ]),
            vec![],
        )]);

        let clip1 = MultiPolygon::new(vec![Polygon::new(
            LineString(vec![
                Coord { x: 190.0, y: 210.0 },
                Coord { x: 240.0, y: 210.0 },
                Coord { x: 240.0, y: 130.0 },
                Coord { x: 190.0, y: 130.0 },
            ]),
            vec![LineString(vec![
                Coord { x: 215.0, y: 160.0 },
                Coord { x: 230.0, y: 190.0 },
                Coord { x: 200.0, y: 190.0 },
            ])],
        )]);

        let clip2 = MultiPolygon::new(vec![Polygon::new(
            LineString(vec![
                Coord { x: 215.0, y: 160.0 },
                Coord { x: 230.0, y: 190.0 },
                Coord { x: 200.0, y: 190.0 },
            ]),
            vec![],
        )]);

        let result = calculate_leftover(&subject, &vec![clip1, clip2]);
        let expected = MultiPolygon::new(vec![
            Polygon::new(
                LineString(vec![
                    Coord { x: 190.0, y: 200.0 },
                    Coord { x: 180.0, y: 200.0 },
                    Coord { x: 180.0, y: 150.0 },
                    Coord { x: 190.0, y: 150.0 },
                    Coord { x: 190.0, y: 200.0 },
                ]),
                vec![],
            ),
            Polygon::new(
                LineString(vec![
                    Coord { x: 260.0, y: 200.0 },
                    Coord { x: 240.0, y: 200.0 },
                    Coord { x: 240.0, y: 150.0 },
                    Coord { x: 260.0, y: 150.0 },
                    Coord { x: 260.0, y: 200.0 },
                ]),
                vec![],
            ),
        ]);
        assert_eq!(expected, result);
        Ok(())
    }
    #[test]
    #[should_panic(expected = "subject can only be polygon or multipolygon")]
    fn error_subject_not_polygon() {
        let subject = LineString(vec![
            Coord { x: 180.0, y: 200.0 },
            Coord { x: 260.0, y: 200.0 },
            Coord { x: 260.0, y: 150.0 },
            Coord { x: 180.0, y: 150.0 },
        ]);

        let clip1 = MultiPolygon::new(vec![Polygon::new(
            LineString(vec![
                Coord { x: 190.0, y: 210.0 },
                Coord { x: 240.0, y: 210.0 },
                Coord { x: 240.0, y: 130.0 },
                Coord { x: 190.0, y: 130.0 },
            ]),
            vec![LineString(vec![
                Coord { x: 215.0, y: 160.0 },
                Coord { x: 230.0, y: 190.0 },
                Coord { x: 200.0, y: 190.0 },
            ])],
        )]);

        let clip2 = MultiPolygon::new(vec![Polygon::new(
            LineString(vec![
                Coord { x: 215.0, y: 160.0 },
                Coord { x: 230.0, y: 190.0 },
                Coord { x: 200.0, y: 190.0 },
            ]),
            vec![],
        )]);

        let _result = calculate_leftover(&subject, &vec![clip1, clip2]);
    }
}
