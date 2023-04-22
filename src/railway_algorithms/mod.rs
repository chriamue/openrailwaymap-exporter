//! Module `railway_algorithms` provides algorithms for working with railway networks.
//!
//! The module contains the `PathFinding` trait, which offers methods to calculate the
//! shortest path distance, the shortest path as a list of node IDs, and the shortest
//! path as a list of edge IDs for railway networks.

/// The `PathFinding` trait is implemented for the `RailwayGraph` type, allowing users
/// to perform pathfinding operations on railway graphs.
mod path_finding;
use crate::algorithms::points_in_front;
use crate::prelude::{RailwayEdge, RailwayGraph};
use geo::Coord;

use geo::algorithm::euclidean_distance::EuclideanDistance;
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

    /// Returns the next reachable node on the shortest path
    pub fn get_next_node(&self, current: i64, target: i64) -> Option<i64> {
        let path = self.shortest_path_nodes(current, target)?;
        path.get(1).copied()
    }
}

impl RailwayEdge {
    /// Calculates a new position on the edge based on the given parameters.
    ///
    /// # Arguments
    ///
    /// * `current_location` - A `Coord<f64>` representing the current location on the edge.
    /// * `distance_to_travel` - A `f64` representing the distance to travel along the edge from the current location.
    /// * `direction_coord` - A `Coord<f64>` representing the target direction along the edge.
    ///
    /// # Returns
    ///
    /// A `Coord<f64>` representing the new position on the edge after traveling the specified distance in the given direction.
    ///
    pub fn position_on_edge(
        &self,
        current_location: Coord<f64>,
        distance_to_travel: f64,
        direction_coord: Coord<f64>,
    ) -> Coord<f64> {
        // Get the points in front of the current_location in the direction of direction_coord
        let points_in_front = points_in_front(&self.path, current_location, direction_coord);

        // If there are no points in front, return the current_location
        if points_in_front.is_empty() {
            return current_location;
        }

        // Calculate the remaining distance to travel
        let mut remaining_distance = distance_to_travel;

        // Iterate through the points in front and find the point where the remaining_distance is reached
        let mut current_point = current_location;
        let mut new_position = current_location;

        for next_point in points_in_front {
            let segment_distance = current_point.euclidean_distance(&next_point);

            if remaining_distance < segment_distance {
                let ratio = remaining_distance / segment_distance;
                new_position.x = current_point.x + ratio * (next_point.x - current_point.x);
                new_position.y = current_point.y + ratio * (next_point.y - current_point.y);
                break;
            }

            current_point = next_point;
            remaining_distance -= segment_distance;
        }
        new_position
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use geo::{coord, line_string};

    use crate::importer::overpass_importer::{
        from_railway_elements, Coordinate, ElementType, RailwayElement,
    };
    use std::collections::HashMap;

    #[test]
    fn test_position_on_edge() {
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

        let current_position1 = coord! { x: 0.0, y: 0.0 };
        let current_position2 = coord! { x: 0.0, y: 20.0 };
        let distance1 = 15.0;
        let direction1 = coord! { x: 0.0, y: 20.0 };
        let direction2 = coord! { x: 100.0, y: 100.0 };

        let new_position1 = edge.position_on_edge(current_position1, distance1, direction1);
        let new_position2 = edge.position_on_edge(current_position2, distance1, direction2);

        assert_eq!(new_position1, coord! { x: 0.0, y: 15.0 });
        assert_relative_eq!(new_position2, coord! { x: 12.9, y: 27.7 }, epsilon = 0.1);

        let current_position3 = coord! { x: 50.0, y: 50.0 };
        let distance2 = 25.0;

        let new_position3 = edge.position_on_edge(current_position3, distance2, direction1);
        let new_position4 = edge.position_on_edge(current_position3, distance2, direction2);

        assert_relative_eq!(new_position3, coord! { x: 32.3, y: 32.3 }, epsilon = 0.1);
        assert_relative_eq!(new_position4, coord! { x: 67.7, y: 67.7 }, epsilon = 0.1);
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
    fn test_get_next_node() {
        let railway_graph = from_railway_elements(&test_elements());

        assert_eq!(railway_graph.get_next_node(1, 2), Some(2));
        assert_eq!(railway_graph.get_next_node(1, 3), Some(2));
        assert_eq!(railway_graph.get_next_node(2, 3), Some(3));
        assert_eq!(railway_graph.get_next_node(1, 4), None);
    }
}
