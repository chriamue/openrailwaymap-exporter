use cucumber::{given, then, when, World};
use geo::Coord;
use serde_json::Value;
use std::fs;
use std::path::Path;
use uom::si::f64::Length;

use openrailwaymap_exporter::importer::overpass_importer::OverpassImporter;
use openrailwaymap_exporter::importer::RailwayGraphImporter;
use openrailwaymap_exporter::prelude::RailwayEdge;
use openrailwaymap_exporter::prelude::RailwayGraph;

pub mod steps;

#[derive(World, Debug)]
pub struct BddWorld {
    json: Value,
    railway_graph: RailwayGraph,
    edge: Option<RailwayEdge>,
    current_location: Option<Coord<f64>>,
    distance_to_travel: Option<Length>,
    direction_coord: Option<Coord<f64>>,
    new_position: Option<Coord<f64>>,
}

impl std::default::Default for BddWorld {
    fn default() -> BddWorld {
        BddWorld {
            json: Value::Null,
            railway_graph: test_graph_vilbel(),
            edge: None,
            current_location: None,
            distance_to_travel: None,
            direction_coord: None,
            new_position: None,
        }
    }
}

pub fn test_json_vilbel() -> Value {
    serde_json::from_slice(include_bytes!("../src/tests/res/vilbel.json"))
        .expect("Failed to deserialize the JSON data")
}

pub fn test_graph_vilbel() -> RailwayGraph {
    OverpassImporter::import(&test_json_vilbel()).unwrap()
}

#[given(expr = "the JSON data from {string}")]
async fn given_json_data(w: &mut BddWorld, file_path: String) {
    let json_data =
        fs::read_to_string(Path::new(&file_path)).expect("Failed to read the JSON data from file");

    w.json = serde_json::from_str(&json_data).expect("Failed to deserialize the JSON data");
}

#[when("the railway graph is imported")]
async fn when_import_railway_graph(w: &mut BddWorld) {
    w.railway_graph = OverpassImporter::import(&w.json).unwrap();
}

#[then(expr = "the graph should have {int} nodes")]
async fn then_graph_should_have_nodes(w: &mut BddWorld, expected_nodes: usize) {
    assert_eq!(w.railway_graph.graph.node_count(), expected_nodes);
}

#[then(expr = "the graph should have {int} edges")]
async fn then_graph_should_have_edges(w: &mut BddWorld, expected_edges: usize) {
    assert_eq!(w.railway_graph.graph.edge_count(), expected_edges);
}

#[tokio::main]
async fn main() {
    BddWorld::run("tests/features").await;
}
