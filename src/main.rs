use leftover::leftover_geo_json_areas;
mod leftover;

fn main() {
    let subject = r#"{
        "type": "MultiPolygon",
        "coordinates": [[[[180.0, 200.0 ],[260.0, 200.0 ],[260.0, 150.0 ],[180.0, 150.0 ],[180.0, 200.0 ]]]]
    }"#;

    let clippers = r#"[
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
    ]"#;
    let result =
        leftover_geo_json_areas(subject, clippers).expect("leftover calculated successfully");
    println!("{}", result);
}
