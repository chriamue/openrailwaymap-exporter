use geoutils::Location;
use petgraph::{stable_graph::NodeIndex, Graph, Undirected};
use std::collections::HashMap;

use crate::railway_element::{ElementType, RailwayElement};
use crate::railway_processing::create_nodes;
use crate::Coordinate;

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

        let nodes = create_nodes(elements);
        for node in nodes {
            let node_index = graph.add_node(node.clone());
            node_indices.insert(node.id, node_index);
        }

        for element in elements.iter() {
            if let ElementType::Way = element.element_type {
                if let (Some(nodes_ids), Some(geometry)) = (&element.nodes, &element.geometry) {
                    let length = calculate_geometry_length(geometry);

                    for i in 0..(nodes_ids.len() - 1) {
                        let node_id = nodes_ids[i];
                        let next_node_id = nodes_ids[i + 1];

                        if let (Some(&node_index), Some(&next_node_index)) =
                            (node_indices.get(&node_id), node_indices.get(&next_node_id))
                        {
                            graph.add_edge(
                                node_index,
                                next_node_index,
                                RailwayEdge {
                                    id: element.id,
                                    length,
                                },
                            );
                        }
                    }
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
                    RailwayEdge {
                        id: 0,
                        length: distance,
                    },
                );
            } else {
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

/// Calculate the total length of a sequence of coordinates by summing the
/// distance between consecutive coordinates.
///
/// # Arguments
///
/// * `geometry` - A slice of `Coordinate` values representing a sequence of connected points.
///
/// # Examples
///
/// ```
/// use openrailwaymap_exporter::Coordinate;
/// use openrailwaymap_exporter::railway_graph::calculate_geometry_length;
///
/// let geometry = vec![
///     Coordinate { lat: 1.0, lon: 1.0 },
///     Coordinate { lat: 2.0, lon: 1.0 },
///     Coordinate { lat: 2.0, lon: 2.0 },
/// ];
///
/// let length = calculate_geometry_length(&geometry);
/// assert_eq!(length, 221827.195);
/// ```
pub fn calculate_geometry_length(geometry: &[Coordinate]) -> f64 {
    let mut length = 0.0;
    for i in 0..(geometry.len() - 1) {
        let coord_a = &geometry[i];
        let coord_b = &geometry[i + 1];
        length += calculate_distance(coord_a.lat, coord_a.lon, coord_b.lat, coord_b.lon);
    }
    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_geometry_length() {
        let geometry = vec![
            Coordinate { lat: 1.0, lon: 1.0 },
            Coordinate { lat: 2.0, lon: 1.0 },
            Coordinate { lat: 2.0, lon: 2.0 },
        ];

        let length = calculate_geometry_length(&geometry);
        assert_eq!((length * 100.0).round() / 100.0, 221827.2); // Compare with rounded value
    }
}
