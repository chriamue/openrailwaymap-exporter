//! Module `railway_algorithms` provides algorithms for working with railway networks.
//!
//! The module contains the `PathFinding` trait, which offers methods to calculate the
//! shortest path distance, the shortest path as a list of node IDs, and the shortest
//! path as a list of edge IDs for railway networks.

/// The `PathFinding` trait is implemented for the `RailwayGraph` type, allowing users
/// to perform pathfinding operations on railway graphs.
mod path_finding;
use crate::prelude::RailwayGraph;
pub use path_finding::PathFinding;
use petgraph::visit::Bfs;

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
    pub fn reachable_nodes(&self, start_node_id: i64) -> Vec<i64> {
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
}

#[cfg(test)]
pub mod tests {
    use crate::importer::overpass_importer::{
        from_railway_elements, Coordinate, ElementType, RailwayElement,
    };

    use std::collections::HashMap;

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
}
