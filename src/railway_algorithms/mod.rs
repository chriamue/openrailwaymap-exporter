//! Module `railway_algorithms` provides algorithms for working with railway networks.
//!
//! The module contains the `PathFinding` trait, which offers methods to calculate the
//! shortest path distance, the shortest path as a list of node IDs, and the shortest
//! path as a list of edge IDs for railway networks.

/// The `PathFinding` trait is implemented for the `RailwayGraph` type, allowing users
/// to perform pathfinding operations on railway graphs.
mod path_finding;
mod railway_edge_algos;

use crate::{
    prelude::RailwayGraph,
    types::{EdgeId, NodeId},
};
pub use path_finding::PathFinding;
use petgraph::visit::Bfs;

pub use railway_edge_algos::RailwayEdgeAlgos;

impl RailwayGraph {
    /// Find all reachable nodes from the given start node in the railway graph.
    ///
    /// This function performs a breadth-first search from the given start node and returns a
    /// vector of reachable node IDs.
    ///
    /// # Arguments
    ///
    /// * `start_node_id` - The ID of the start node.
    ///
    /// # Returns
    ///
    /// A `Vec<i64>` containing the IDs of all nodes reachable from the start node.
    /// If the start node ID is not found in the graph, an empty vector is returned.
    pub fn reachable_nodes(&self, start_node_id: NodeId) -> Vec<NodeId> {
        if let Some(start_index) = self.node_indices.get(&start_node_id) {
            let mut reachable_nodes = Vec::new();
            let mut bfs = Bfs::new(&self.graph, *start_index);

            while let Some(visited_node_index) = bfs.next(&self.graph) {
                let visited_node_id = self.graph.node_weight(visited_node_index).unwrap().id;
                if visited_node_id != start_node_id {
                    reachable_nodes.push(visited_node_id);
                }
            }

            reachable_nodes
        } else {
            vec![]
        }
    }

    /// Find all reachable edges from the given start node in the railway graph.
    ///
    /// This function performs a breadth-first search from the given start node and returns a
    /// vector of reachable edge IDs.
    ///
    /// # Arguments
    ///
    /// * `start_node_id` - The ID of the start node.
    ///
    /// # Returns
    ///
    /// A `Vec<i64>` containing the IDs of all edges reachable from the start node.
    /// If the start node ID is not found in the graph, an empty vector is returned.
    pub fn reachable_edges(&self, start_node_id: NodeId) -> Vec<EdgeId> {
        if let Some(start_index) = self.node_indices.get(&start_node_id) {
            let mut reachable_edges = Vec::new();
            let mut bfs = Bfs::new(&self.graph, *start_index);

            while let Some(visited_node_index) = bfs.next(&self.graph) {
                let visited_node_edges = self.graph.edges(visited_node_index);
                for edge in visited_node_edges {
                    let edge_id = edge.weight().id;
                    if !reachable_edges.contains(&edge_id) {
                        reachable_edges.push(edge_id);
                    }
                }
            }

            reachable_edges
        } else {
            vec![]
        }
    }

    /// Returns the next reachable node on the shortest path
    pub fn get_next_node(&self, current: NodeId, target: NodeId) -> Option<NodeId> {
        let path = self.shortest_path_nodes(current, target)?;
        path.get(1).copied()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use geo::{coord, line_string};
    use uom::si::{f64::Length, length::meter};

    use crate::{
        importer::overpass_importer::{
            from_railway_elements, Coordinate, ElementType, RailwayElement,
        },
        prelude::RailwayEdge,
    };
    use std::collections::HashMap;

    #[test]
    fn test_distance_to_end() {
        let edge = RailwayEdge {
            id: 1,
            length: 1166.0,
            path: line_string![
                coord! { x: 13.377054, y: 52.516250 }, // Brandenburg Gate, Berlin
                coord! { x: 13.378685, y: 52.520165 }, // Reichstag Building, Berlin
                coord! { x: 13.384733, y: 52.522464 }, // Berlin Central Station
            ],
            source: 1,
            target: 2,
        };

        let current_position1 = coord! { x: 13.377054, y: 52.516250 }; // Brandenburg Gate, Berlin
        let direction1 = coord! { x: 13.378685, y: 52.520165 }; // Reichstag Building, Berlin
        let distance_to_end1 = edge.distance_to_end(current_position1, direction1);
        let expected_distance1 = Length::new::<meter>(1796.0); // Approx. distance between Brandenburg Gate and Berlin Central Station
        assert_relative_eq!(
            distance_to_end1.get::<meter>(),
            expected_distance1.get::<meter>(),
            epsilon = 10.0
        );

        let current_position2 = coord! { x: 13.378685, y: 52.520165 }; // Reichstag Building, Berlin
        let direction2 = coord! { x: 13.384733, y: 52.522464 }; // Berlin Central Station
        let distance_to_end2 = edge.distance_to_end(current_position2, direction2);
        let expected_distance2 = Length::new::<meter>(480.0); // Approx. distance between Reichstag Building and Berlin Central Station
        assert_relative_eq!(
            distance_to_end2.get::<meter>(),
            expected_distance2.get::<meter>(),
            epsilon = 10.0
        );
    }

    #[test]
    fn test_position_on_edge() {
        let edge = RailwayEdge {
            id: 1,
            length: 100.0,
            path: line_string![
                coord! { x: 8.6821, y: 50.1109 },
                coord! { x: 8.6825, y: 50.1112 },
                coord! { x: 8.6830, y: 50.1115 },
                coord! { x: 8.6835, y: 50.1118 },
            ],
            source: 1,
            target: 2,
        };

        let current_position1 = coord! { x: 8.6821, y: 50.1109 };
        let current_position2 = coord! { x: 8.6825, y: 50.1112 };
        let distance1 = Length::new::<meter>(15.0);
        let direction1 = coord! { x: 8.6825, y: 50.1112 };
        let direction2 = coord! { x: 8.6835, y: 50.1118 };

        let new_position1 = edge.position_on_edge(current_position1, distance1, direction1);
        let new_position2 = edge.position_on_edge(current_position2, distance1, direction2);

        assert_relative_eq!(new_position1.x, 8.6823, epsilon = 0.001);
        assert_relative_eq!(new_position1.y, 50.1111, epsilon = 0.001);
        assert_relative_eq!(new_position2.x, 8.6830, epsilon = 0.001);
        assert_relative_eq!(new_position2.y, 50.1115, epsilon = 0.001);

        let current_position3 = coord! { x: 8.6830, y: 50.1115 };

        let distance2 = Length::new::<meter>(25.0);
        let new_position3 = edge.position_on_edge(current_position3, distance2, direction1);
        let new_position4 = edge.position_on_edge(current_position3, distance2, direction2);

        assert_relative_eq!(new_position3.x, 8.6827, epsilon = 0.0001);
        assert_relative_eq!(new_position3.y, 50.1113, epsilon = 0.0001);
        assert_relative_eq!(new_position4.x, 8.6833, epsilon = 0.0001);
        assert_relative_eq!(new_position4.y, 50.1116, epsilon = 0.0001);

        let current_position4 = coord! { x: 8.6830, y: 50.1115 };
        let distance3 = Length::new::<meter>(100.0);

        let new_position5 = edge.position_on_edge(current_position4, distance3, direction2);

        assert_relative_eq!(new_position5.x, 8.6840, epsilon = 0.0001);
        assert_relative_eq!(new_position5.y, 50.1121, epsilon = 0.0001);
    }

    pub fn test_elements() -> Vec<RailwayElement> {
        vec![
            RailwayElement {
                id: 1,
                element_type: ElementType::Node,
                lat: Some(50.1109),
                lon: Some(8.6821),
                tags: Some(HashMap::new()),
                nodes: None,
                geometry: None,
            },
            RailwayElement {
                id: 2,
                element_type: ElementType::Node,
                lat: Some(50.1209),
                lon: Some(8.6921),
                tags: Some(HashMap::new()),
                nodes: None,
                geometry: None,
            },
            RailwayElement {
                id: 3,
                element_type: ElementType::Node,
                lat: Some(50.1309),
                lon: Some(8.6721),
                tags: Some(HashMap::new()),
                nodes: None,
                geometry: None,
            },
            RailwayElement {
                id: 4,
                element_type: ElementType::Way,
                lat: None,
                lon: None,
                tags: Some(HashMap::new()),
                nodes: Some(vec![1, 2]),
                geometry: Some(vec![
                    Coordinate {
                        lat: 50.1109,
                        lon: 8.6821,
                    },
                    Coordinate {
                        lat: 50.1209,
                        lon: 8.6921,
                    },
                ]),
            },
            RailwayElement {
                id: 5,
                element_type: ElementType::Way,
                lat: None,
                lon: None,
                tags: Some(HashMap::new()),
                nodes: Some(vec![2, 3]),
                geometry: Some(vec![
                    Coordinate {
                        lat: 50.1209,
                        lon: 8.6921,
                    },
                    Coordinate {
                        lat: 50.1309,
                        lon: 8.6721,
                    },
                ]),
            },
        ]
    }
    #[test]
    fn test_reachable_nodes() {
        let railway_graph = from_railway_elements(&test_elements());

        let start_node_id = 1;
        let reachable_nodes = railway_graph.reachable_nodes(start_node_id);
        assert_eq!(reachable_nodes, vec![2, 3]);
    }

    #[test]
    fn test_reachable_edges() {
        let railway_graph = from_railway_elements(&test_elements());

        let start_node_id = 1;
        let reachable_edges = railway_graph.reachable_edges(start_node_id);
        assert_eq!(reachable_edges, vec![4, 5]);
    }

    #[test]
    fn test_get_next_node() {
        let railway_graph = from_railway_elements(&test_elements());

        assert_eq!(railway_graph.get_next_node(1, 2), Some(2));
        assert_eq!(railway_graph.get_next_node(1, 3), Some(2));
        assert_eq!(railway_graph.get_next_node(2, 3), Some(3));
        assert_eq!(railway_graph.get_next_node(1, 4), None);
    }
}
