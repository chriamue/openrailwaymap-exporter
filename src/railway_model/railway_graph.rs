use geoutils::Location;
use petgraph::{stable_graph::NodeIndex, Graph, Undirected};
use std::collections::HashMap;

use crate::railway_element::RailwayElement;

use super::{RailwayEdge, RailwayNode};

#[derive(Debug)]
pub struct RailwayGraph {
    pub graph: Graph<RailwayNode, RailwayEdge, Undirected>,
    node_indices: HashMap<i64, NodeIndex>,
}

impl RailwayGraph {
    pub fn from_railway_elements(elements: &Vec<RailwayElement>) -> Self {
        let mut graph = Graph::<RailwayNode, RailwayEdge, Undirected>::new_undirected();
        let mut node_indices = HashMap::new();

        let empty_nodes_vec = Vec::new();
        let empty_geometry_vec = Vec::new();

        for element in elements.iter() {
            let nodes_ids = element.nodes.as_ref().unwrap_or(&empty_nodes_vec);
            let geometry = element.geometry.as_ref().unwrap_or(&empty_geometry_vec);

            for (i, &node_id) in nodes_ids.iter().enumerate() {
                let coord = &geometry[i];

                let node_index = *node_indices.entry(node_id).or_insert_with(|| {
                    graph.add_node(RailwayNode {
                        id: node_id,
                        lat: coord.lat,
                        lon: coord.lon,
                    })
                });

                if i < nodes_ids.len() - 1 {
                    let next_node_id = nodes_ids[i + 1];
                    let next_coord = &geometry[i + 1];
                    let distance =
                        calculate_distance(coord.lat, coord.lon, next_coord.lat, next_coord.lon);

                    let next_node_index = *node_indices.entry(next_node_id).or_insert_with(|| {
                        graph.add_node(RailwayNode {
                            id: next_node_id,
                            lat: next_coord.lat,
                            lon: next_coord.lon,
                        })
                    });

                    graph.add_edge(
                        node_index,
                        next_node_index,
                        RailwayEdge {
                            id: element.id,
                            distance,
                        },
                    );
                }
            }
        }

        let connections = find_connected_elements(&elements);
        for (source_id, target_id) in connections {
            if let (Some(source_index), Some(target_index)) =
                (node_indices.get(&source_id), node_indices.get(&target_id))
            {
                let source_node = &graph[*source_index];
                let target_node = &graph[*target_index];
                let distance = calculate_distance(
                    source_node.lat,
                    source_node.lon,
                    target_node.lat,
                    target_node.lon,
                );

                graph.add_edge(
                    *source_index,
                    *target_index,
                    RailwayEdge { id: 0, distance },
                );
            } else {
                // Handle the case where either source_index or target_index is not found in node_indices
                // You can log a warning, ignore the edge, or handle it in any other way you deem appropriate
                println!("{} not found", source_id);
            }
        }

        RailwayGraph {
            graph,
            node_indices,
        }
    }
}

fn find_connected_elements(elements: &[RailwayElement]) -> Vec<(i64, i64)> {
    let mut connections: Vec<(i64, i64)> = Vec::new();

    for i in 0..elements.len() {
        let elem_a = &elements[i];
        if let Some(nodes_a) = &elem_a.nodes {
            for j in (i + 1)..elements.len() {
                let elem_b = &elements[j];
                if let Some(nodes_b) = &elem_b.nodes {
                    let common_nodes: Vec<_> = nodes_a
                        .iter()
                        .filter(|node_a| nodes_b.contains(node_a))
                        .collect();
                    if !common_nodes.is_empty() {
                        connections.push((elem_a.id, elem_b.id));
                    }
                }
            }
        }
    }

    connections
}

fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let location1 = Location::new(lat1, lon1);
    let location2 = Location::new(lat2, lon2);
    let distance = location1.distance_to(&location2).unwrap();
    distance.meters()
}
