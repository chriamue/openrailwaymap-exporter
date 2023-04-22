use cucumber::codegen::Regex;
use cucumber::{gherkin::Step, given, then, when};
use geo::Coord;

use crate::BddWorld;
use approx::assert_relative_eq;

#[given(regex = "a RailwayEdge with the following properties:")]
async fn given_railway_edge_with_properties(world: &mut BddWorld, step: &Step) {
    let table = step.table.as_ref().unwrap();
    let row = &table.rows[1];

    println!("{:?}", row);

    let id = row[0].parse::<i64>().unwrap();
    let length = row[1].parse::<f64>().unwrap();
    let path_coordinates_str = &row[2];
    let source = row[3].parse::<i64>().unwrap();
    let target = row[4].parse::<i64>().unwrap();

    let re = Regex::new(r"\(([\d.]+),\s*([\d.]+)\)").unwrap();
    let path_coordinates: Vec<Coord<f64>> = re
        .captures_iter(path_coordinates_str)
        .map(|cap| Coord {
            x: cap[1].parse::<f64>().unwrap(),
            y: cap[2].parse::<f64>().unwrap(),
        })
        .collect();

    world.edge = Some(openrailwaymap_exporter::prelude::RailwayEdge {
        id,
        length,
        path: geo_types::LineString::from(path_coordinates),
        source,
        target,
    });
}

#[given(regex = r"a current location at \(([\d.]+),\s*([\d.]+)\)")]
async fn given_current_location_at(world: &mut BddWorld, x: f64, y: f64) {
    world.current_location = Some(Coord { x, y });
}

#[given(regex = "a distance to travel of (\\d+\\.?\\d*) meters")]
async fn given_distance_to_travel_of_meters(world: &mut BddWorld, distance: f64) {
    world.distance_to_travel = Some(distance);
}

#[given(regex = r"a direction coordinate of \(([\d.]+),\s*([\d.]+)\)")]
async fn given_direction_coordinate(world: &mut BddWorld, x: f64, y: f64) {
    world.direction_coord = Some(Coord { x, y });
}

#[when("I call position_on_edge with the given parameters")]
async fn when_call_position_on_edge(world: &mut BddWorld) {
    let edge = world.edge.as_ref().expect("RailwayEdge is not set");
    let current_location = world
        .current_location
        .expect("Current location is not set");
    let distance_to_travel = world.distance_to_travel.expect("Distance to travel is not set");
    let direction_coord = world
        .direction_coord
        .expect("Direction coordinate is not set");

    let position = edge.position_on_edge(current_location, distance_to_travel, direction_coord);
    world.new_position = Some(position);
}

#[then(regex = r"the new position should be approximately \(([\d.]+),\s*([\d.]+)\)")]
async fn then_new_position_approximately(world: &mut BddWorld, x: f64, y: f64) {
    let expected_position = Coord { x, y };
    let new_position = world.new_position.expect("New position is not set");

    assert_relative_eq!(new_position.x, expected_position.x, epsilon = 0.0001);
    assert_relative_eq!(new_position.y, expected_position.y, epsilon = 0.0001);
}
