use serde_json::Value;

use crate::{count_way_elements, RailwayElement, RailwayGraph};

pub fn test_json_1() -> Value {
    serde_json::from_slice(include_bytes!("res/test1.json"))
        .expect("Failed to deserialize the JSON data")
}

#[test]
fn test_load_railway_graph_from_test1() {
    let railway_elements = RailwayElement::from_json(&test_json_1()).unwrap();
    let expected_edges_count = count_way_elements(&railway_elements);
    let railway_graph = RailwayGraph::from_railway_elements(&railway_elements);

    println!("{:?}", railway_graph.graph);
    assert_eq!(railway_graph.graph.node_count(), 4);
    assert_eq!(expected_edges_count, 5);
    // assert_eq!(railway_graph.graph.edge_count(), expected_edges_count);
    assert_eq!(railway_graph.graph.edge_count(), 3);
}
