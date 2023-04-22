use criterion::{black_box, criterion_group, criterion_main, Criterion};

use geo::{coord, line_string};
use openrailwaymap_exporter::{
    importer::overpass_importer::{find_next_existing_node, from_railway_elements, RailwayElement},
    railway_algorithms::PathFinding,
    railway_model::RailwayEdge,
};
use petgraph::stable_graph::NodeIndex;
use std::collections::HashMap;

fn railway_elements() -> Vec<RailwayElement> {
    let test_data = serde_json::from_slice(include_bytes!("../src/tests/res/vilbel.json"))
        .expect("Failed to deserialize the JSON data");
    RailwayElement::from_json(&test_data).unwrap()
}

fn benchmark_from_railway_elements(c: &mut Criterion) {
    let elements = railway_elements();
    c.bench_function("from_railway_elements", |b| {
        b.iter(|| from_railway_elements(black_box(&elements)))
    });
}

fn find_next_existing_node_benchmark(c: &mut Criterion) {
    let node_ids = vec![1, 3, 5];
    let mut node_indices = HashMap::new();
    node_indices.insert(1, NodeIndex::new(0));
    node_indices.insert(3, NodeIndex::new(1));
    node_indices.insert(5, NodeIndex::new(2));

    c.bench_function("find_next_existing_node", |b| {
        b.iter(|| {
            let start = black_box(Some(1));
            let node_ids = black_box(&node_ids);
            let node_indices = black_box(&node_indices);

            find_next_existing_node(start, node_ids, node_indices)
        })
    });
}

fn shortest_path_edges_benchmark(c: &mut Criterion) {
    let elements = railway_elements();
    let railway_graph = from_railway_elements(&elements);

    c.bench_function("shortest_path_edges", |b| {
        b.iter(|| {
            let start = black_box(6204567489);
            let end = black_box(6204567501);
            assert!(!railway_graph
                .shortest_path_edges(start, end)
                .unwrap()
                .is_empty());
        })
    });
}

fn reachable_nodes_benchmark(c: &mut Criterion) {
    let elements = railway_elements();
    let railway_graph = from_railway_elements(&elements);

    c.bench_function("reachable_nodes", |b| {
        b.iter(|| {
            let start = black_box(6204567489);
            let reachable_nodes = railway_graph.reachable_nodes(start);
            assert!(!reachable_nodes.is_empty());
        })
    });
}

fn update_position_benchmark(c: &mut Criterion) {
    let edge = RailwayEdge {
        id: 1,
        length: 100.0,
        path: line_string![
            coord! { x: 0.0, y: 0.0 },
            coord! { x: 0.0, y: 20.0 },
            coord! { x: 50.0, y: 50.0 },
            coord! { x: 100.0, y: 100.0 },
        ],
        source: 1,
        target: 2,
    };

    let current_location = coord! { x: 10.0, y: 20.0 };
    let distance_to_travel = 25.0;
    let direction_coord = coord! { x: 100.0, y: 100.0 };

    c.bench_function("update_position", |b| {
        b.iter(|| {
            let updated_position = edge.position_on_edge(
                black_box(current_location),
                black_box(distance_to_travel),
                black_box(direction_coord),
            );
            assert_ne!(updated_position, current_location);
        })
    });
}

criterion_group!(
    benches,
    benchmark_from_railway_elements,
    find_next_existing_node_benchmark,
    shortest_path_edges_benchmark,
    reachable_nodes_benchmark,
    update_position_benchmark
);
criterion_main!(benches);
