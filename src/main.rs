use geo::geometry::{Coord, LineString, MultiPolygon, Polygon};
use geo_clipper::Clipper;

fn calculate_leftover(subject: MultiPolygon, clippers: Vec<MultiPolygon>) -> MultiPolygon {
    clippers
        .iter()
        .fold(subject, |leftover, clip| leftover.difference(clip, 1.0))
}

fn main() {
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
            Coord { x: 190.0, y: 210.0 },
            Coord { x: 240.0, y: 210.0 },
            Coord { x: 240.0, y: 130.0 },
            Coord { x: 190.0, y: 130.0 },
        ]),
        vec![],
    )]);

    let result = calculate_leftover(subject, vec![clip1, clip2]);
    println!("{:?}", result);
}
