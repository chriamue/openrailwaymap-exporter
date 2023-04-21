use std::fs;
use std::path::Path;
use std::time::Duration;

use cucumber::{given, then, when, World as _};
use serde_json::Value;
use tokio::time::sleep;

use openrailwaymap_exporter::importer::overpass_importer::OverpassImporter;
use openrailwaymap_exporter::importer::RailwayGraphImporter;
use openrailwaymap_exporter::prelude::RailwayGraph;

#[derive(cucumber::World, Debug)]
struct MyWorld {
    json: Value,
    railway_graph: RailwayGraph,
}

impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        MyWorld {
            json: Value::Null,
            railway_graph: test_graph_vilbel(),
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
async fn given_json_data(w: &mut MyWorld, file_path: String) {
    sleep(Duration::from_secs(2)).await;

    let json_data =
        fs::read_to_string(Path::new(&file_path)).expect("Failed to read the JSON data from file");

    w.json = serde_json::from_str(&json_data).expect("Failed to deserialize the JSON data");
}

#[when("the railway graph is imported")]
async fn when_import_railway_graph(w: &mut MyWorld) {
    sleep(Duration::from_secs(2)).await;

    w.railway_graph = OverpassImporter::import(&w.json).unwrap();
}

#[then(expr = "the graph should have {int} nodes")]
async fn then_graph_should_have_nodes(w: &mut MyWorld, expected_nodes: usize) {
    sleep(Duration::from_secs(2)).await;

    assert_eq!(w.railway_graph.graph.node_count(), expected_nodes);
}

#[then(expr = "the graph should have {int} edges")]
async fn then_graph_should_have_edges(w: &mut MyWorld, expected_edges: usize) {
    sleep(Duration::from_secs(2)).await;

    assert_eq!(w.railway_graph.graph.edge_count(), expected_edges);
}

#[cfg(feature = "cucumber")]
#[tokio::main]
async fn main() {
    MyWorld::run("tests/features").await;
}

#[cfg(not(feature = "cucumber"))]
#[tokio::main]
async fn main() {
    println!("run cucumber tests: cargo test --features cucumber")
}
