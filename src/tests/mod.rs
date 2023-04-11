use std::collections::HashMap;

use petgraph::{Graph, Undirected};
use serde_json::Value;

use crate::{
    importer::overpass_importer::{count_way_elements, RailwayElement},
    prelude::{
        create_nodes, find_next_existing_node, from_railway_elements, RailwayEdge, RailwayNode,
    },
};

pub fn test_json_1() -> Value {
    serde_json::from_slice(include_bytes!("res/test1.json"))
        .expect("Failed to deserialize the JSON data")
}

pub fn test_json_vilbel() -> Value {
    serde_json::from_slice(include_bytes!("res/vilbel.json"))
        .expect("Failed to deserialize the JSON data")
}

#[test]
fn test_load_railway_graph_from_test1() {
    let railway_elements = RailwayElement::from_json(&test_json_1()).unwrap();
    let expected_edges_count = count_way_elements(&railway_elements);
    let railway_graph = from_railway_elements(&railway_elements);

    println!("{:?}", railway_graph.graph);
    assert_eq!(railway_graph.graph.node_count(), 4);
    assert_eq!(expected_edges_count, 5);
    // assert_eq!(railway_graph.graph.edge_count(), expected_edges_count);
    assert_eq!(railway_graph.graph.edge_count(), 3);
}

#[test]
fn test_vilbel_json() {
    let railway_elements = RailwayElement::from_json(&test_json_vilbel()).unwrap();

    assert_eq!(railway_elements.len(), 101);
    let nodes = create_nodes(&railway_elements);
    assert_eq!(nodes.len(), 68);

    let mut graph = Graph::<RailwayNode, RailwayEdge, Undirected>::new_undirected();
    let mut node_indices = HashMap::new();
    for node in &nodes {
        let node_index = graph.add_node(node.clone());
        node_indices.insert(node.id, node_index);
    }

    assert!(node_indices.contains_key(&1257927271));
    assert!(node_indices.contains_key(&1257927356));
    assert_ne!(
        find_next_existing_node(None, &[1257927271], &node_indices),
        (None, None)
    );
    assert_ne!(
        find_next_existing_node(None, &[1257927271, 1257927356], &node_indices),
        (None, None)
    );
    assert_ne!(
        find_next_existing_node(Some(1257927271), &[1257927271, 1257927356], &node_indices),
        (None, None)
    );

    assert!(node_indices.contains_key(&1257927392));
    assert_ne!(
        find_next_existing_node(Some(3130993015), &[3130993015, 1257927392], &node_indices),
        (None, None)
    );

    let railway_graph = from_railway_elements(&railway_elements);
    assert_eq!(railway_graph.graph.node_count(), 68);
    assert_eq!(railway_graph.graph.edge_count(), 68);
}
