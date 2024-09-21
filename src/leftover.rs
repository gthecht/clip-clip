use eyre::Result;
use geo::{GeodesicArea, Geometry, MultiPolygon};
use geo_clipper::Clipper;
use geojson;

use crate::types::{CoverResponse, GeoArea};

/// Given some input subject (either polygon or multipolygon), and a series of other clippers
/// Return the leftover of the subject after intersecting with all the clippers one after the other
pub fn calculate_leftover(subject: Geometry, clippers: Vec<Geometry>) -> Result<MultiPolygon> {
    let multipolygon_subject: MultiPolygon;
    match subject {
        Geometry::MultiPolygon(subject) => multipolygon_subject = subject,
        Geometry::Polygon(subject) => multipolygon_subject = MultiPolygon::new(vec![subject]),
        _ => return Err(eyre::eyre!("subject can only be polygon or multipolygon")),
    }
    clippers.iter().try_fold(
        multipolygon_subject,
        |leftover: MultiPolygon, clip| match clip {
            Geometry::MultiPolygon(clip) => Ok(leftover.difference(clip, 1.0)),
            Geometry::Polygon(clip) => Ok(leftover.difference(clip, 1.0)),
            _ => return Err(eyre::eyre!("clipper can only be polygon or multipolygon")),
        },
    )
}

fn geo_multi_to_geojson_multi(mp: MultiPolygon) -> geojson::Geometry {
    let value = geojson::Value::MultiPolygon(
        mp.0.into_iter()
            .map(|p| {
                let exterior: Vec<Vec<f64>> =
                    p.exterior().0.iter().map(|p| vec![p.x, p.y]).collect();
                let holes: Vec<Vec<Vec<f64>>> = p
                    .interiors()
                    .iter()
                    .map(|ring| ring.0.iter().map(|p| vec![p.x, p.y]).collect())
                    .collect();
                vec![exterior]
                    .into_iter()
                    .chain(holes.into_iter())
                    .collect()
            })
            .collect(),
    );
    geojson::Geometry {
        bbox: None,
        value,
        foreign_members: None,
    }
}

pub fn get_coverage(subject: GeoArea, clippers: Vec<GeoArea>) -> Result<CoverResponse> {
    let subject: Geometry = subject
        .area
        .try_into()
        .expect("expected subject to be valid geojson geometry");
    let clippers: Vec<Geometry> = clippers
        .iter()
        .map(|clip| {
            Geometry::try_from(clip.area.clone())
                .expect("expected clipper to be valid geojson geometry")
        })
        .collect();
    let leftover: MultiPolygon = calculate_leftover(subject.clone(), clippers)?;
    let intersection: MultiPolygon = calculate_leftover(
        subject.clone(),
        vec![Geometry::MultiPolygon(leftover.clone())],
    )?;
    let covered_percentage =
        100.0 * (intersection.geodesic_area_unsigned() / subject.geodesic_area_unsigned());
    let leftover: geojson::Geometry = geo_multi_to_geojson_multi(leftover);
    let intersection: geojson::Geometry = geo_multi_to_geojson_multi(intersection);
    let response = CoverResponse::new(covered_percentage, Some(leftover), Some(intersection), None);
    Ok(response)
}

pub fn leftover_geo_json_areas(subject: &str, clippers: &str) -> Result<String> {
    let subject: geojson::Geometry = serde_json::from_str(subject).unwrap();
    let subject: Geometry = subject
        .try_into()
        .expect("expected subject to be valid geojson geometry");
    let clippers: Vec<geojson::Geometry> = serde_json::from_str(clippers)?;
    let clippers: Vec<Geometry> = clippers
        .iter()
        .map(|clip| {
            Geometry::try_from(clip).expect("expected clipper to be valid geojson geometry")
        })
        .collect();
    let leftover: MultiPolygon = calculate_leftover(subject, clippers)?;
    let leftover: geojson::Geometry = geo_multi_to_geojson_multi(leftover);
    Ok(serde_json::to_string(&leftover)?)
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

        let clip1 = Geometry::Polygon(Polygon::new(
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
        ));

        let clip2 = Geometry::Polygon(Polygon::new(
            LineString(vec![
                Coord { x: 215.0, y: 160.0 },
                Coord { x: 230.0, y: 190.0 },
                Coord { x: 200.0, y: 190.0 },
            ]),
            vec![],
        ));

        let result = calculate_leftover(subject, vec![clip1, clip2])?;
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
        let subject = Geometry::MultiPolygon(MultiPolygon(vec![Polygon::new(
            LineString(vec![
                Coord { x: 180.0, y: 200.0 },
                Coord { x: 260.0, y: 200.0 },
                Coord { x: 260.0, y: 150.0 },
                Coord { x: 180.0, y: 150.0 },
            ]),
            vec![],
        )]));

        let clip1 = Geometry::MultiPolygon(MultiPolygon::new(vec![Polygon::new(
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
        )]));

        let clip2 = Geometry::MultiPolygon(MultiPolygon::new(vec![Polygon::new(
            LineString(vec![
                Coord { x: 215.0, y: 160.0 },
                Coord { x: 230.0, y: 190.0 },
                Coord { x: 200.0, y: 190.0 },
            ]),
            vec![],
        )]));

        let result = calculate_leftover(subject, vec![clip1, clip2])?;
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
        let subject = Geometry::LineString(LineString(vec![
            Coord { x: 180.0, y: 200.0 },
            Coord { x: 260.0, y: 200.0 },
            Coord { x: 260.0, y: 150.0 },
            Coord { x: 180.0, y: 150.0 },
        ]));

        let clip1 = Geometry::MultiPolygon(MultiPolygon::new(vec![Polygon::new(
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
        )]));

        let clip2 = Geometry::MultiPolygon(MultiPolygon::new(vec![Polygon::new(
            LineString(vec![
                Coord { x: 215.0, y: 160.0 },
                Coord { x: 230.0, y: 190.0 },
                Coord { x: 200.0, y: 190.0 },
            ]),
            vec![],
        )]));

        let _result = calculate_leftover(subject, vec![clip1, clip2]).unwrap();
    }

    #[test]
    fn string_input_leftover_test() -> Result<()> {
        let subject = r#"{
                    "type": "MultiPolygon",
                    "coordinates": [[[[180.0, 200.0 ],[260.0, 200.0 ],[260.0, 150.0 ],[180.0, 150.0 ],[180.0, 200.0 ]]]]
            }
        "#;

        let clippers = r#"
            [
                {
                    "type": "MultiPolygon",
                    "coordinates": [
                        [[[190.0, 210.0 ],[240.0, 210.0 ],[240.0, 130.0 ],[190.0, 130.0 ]]],
                        [[[215.0, 160.0 ],[230.0, 190.0 ],[200.0, 190.0 ]]]
                    ]
                },
                {
                    "type": "MultiPolygon",
                    "coordinates": [[[[190.0, 210.0 ],[240.0, 210.0 ],[240.0, 130.0 ],[190.0, 130.0 ]]]]
                }
            ]
        "#;
        let result = leftover_geo_json_areas(subject, clippers)?;
        let expected = r#"{"coordinates":[[[[190.0,200.0],[180.0,200.0],[180.0,150.0],[190.0,150.0],[190.0,200.0]]],[[[260.0,200.0],[240.0,200.0],[240.0,150.0],[260.0,150.0],[260.0,200.0]]]],"type":"MultiPolygon"}"#;
        assert_eq!(expected, result);
        Ok(())
    }
}
