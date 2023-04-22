//! Module `railway_algorithms` provides algorithms for working with railway networks.
//!
//! The module contains the `PathFinding` trait, which offers methods to calculate the
//! shortest path distance, the shortest path as a list of node IDs, and the shortest
//! path as a list of edge IDs for railway networks.

/// The `PathFinding` trait is implemented for the `RailwayGraph` type, allowing users
/// to perform pathfinding operations on railway graphs.
mod path_finding;
use crate::prelude::{RailwayEdge, RailwayGraph};
use geo::{coord, Coord, Point};
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

use geo::algorithm::closest_point::ClosestPoint;
use geo::algorithm::euclidean_distance::EuclideanDistance;

impl RailwayEdge {
    /// Returns a position on the edge given a current position, a distance to a node (source or target),
    /// and the node itself.
    ///
    /// # Arguments
    ///
    /// * `current_position` - The current position as a `Coord<f64>`.
    /// * `distance` - The distance to the given node in meters.
    /// * `direction_node` - The node (source or target) as a `Coord<f64>` where the distance is going to.
    ///
    /// # Returns
    ///
    /// A `Coord<f64>` representing the new position on the edge.
    pub fn position_on_edge(
        &self,
        current_position: Coord<f64>,
        distance: f64,
        direction_node: Coord<f64>,
    ) -> Coord<f64> {
        let p_current_position = Point::new(current_position.x, current_position.y);
        let current_point = match self.path.closest_point(&p_current_position) {
            geo::Closest::Intersection(p) => p,
            geo::Closest::SinglePoint(p) => p,
            geo::Closest::Indeterminate => p_current_position,
        };
        let current_distance = current_point.euclidean_distance(&p_current_position);

        let target_distance = current_distance + distance;
        let mut remaining_distance = target_distance;

        let points: Vec<_> = self.path.points().collect();
        let _source_coord = self.source_coordinate();
        let target_coord = self.target_coordinate();

        let is_forward = direction_node.euclidean_distance(&target_coord)
            < current_position.euclidean_distance(&target_coord);

        let starts: Vec<_> = if is_forward {
            points.iter().collect()
        } else {
            points.iter().rev().collect()
        };
        let ends: Vec<_> = if is_forward {
            points.iter().skip(1).collect()
        } else {
            points.iter().rev().skip(1).collect()
        };

        let segments: Vec<_> = starts
            .iter()
            .zip(ends.iter())
            .map(|(s, e)| (*s, *e))
            .collect();

        let mut previous_point = current_point;

        fn normalize_vector(x: f64, y: f64) -> (f64, f64) {
            let length = (x * x + y * y).sqrt();
            (x / length, y / length)
        }

        for (start, end) in segments {
            let segment_length = start.euclidean_distance(end);

            if remaining_distance < segment_length {
                let (dir_x, dir_y) = normalize_vector(end.x() - start.x(), end.y() - start.y());
                let new_position = Point::new(
                    previous_point.x() + dir_x * remaining_distance,
                    previous_point.y() + dir_y * remaining_distance,
                );
                return coord! { x: new_position.x(), y: new_position.y() };
            } else {
                remaining_distance -= segment_length;
                previous_point = *end;
            }
        }

        direction_node
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use geo::{coord, line_string};

    use crate::importer::overpass_importer::{
        from_railway_elements, Coordinate, ElementType, RailwayElement,
    };

    use std::collections::HashMap;
    #[test]
    fn test_position_on_edge() {
        let edge = RailwayEdge {
            id: 1,
            length: 1500.0,
            path: line_string![
                coord! { x: 0.0, y: 0.0 },
                coord! { x: 0.0, y: 10.0 },
                coord! { x: 10.0, y: 10.0 },
                coord! { x: 10.0, y: 20.0 },
                coord! { x: 20.0, y: 20.0 },
            ],
            source: 1,
            target: 2,
        };

        let current_position = coord! { x: 0.0, y: 5.0 };
        let distance = 7.0;
        let source_node = coord! { x: 0.0, y: 0.0 };
        let target_node = coord! { x: 20.0, y: 20.0 };

        let new_position_to_source = edge.position_on_edge(current_position, distance, source_node);
        let new_position_to_target = edge.position_on_edge(current_position, distance, target_node);

        assert_eq!(new_position_to_source, coord! { x: -7.0, y: 5.0 });
        assert_eq!(new_position_to_target, coord! { x: 0.0, y: 12.0 });

        //let current_position = coord! { x: 10.0, y: 10.0 };
        //let distance = 15.0;

        //let new_position_to_source = edge.position_on_edge(current_position, distance, source_node);
        //let new_position_to_target = edge.position_on_edge(current_position, distance, target_node);

        //assert_eq!(new_position_to_source, coord! { x: 0.0, y: 5.0 });
        //assert_eq!(new_position_to_target, coord! { x: 15.0, y: 20.0 });

        let current_position = coord! { x: 0.0, y: 0.0 };
        let distance = 15.0;

        let new_position_to_target = edge.position_on_edge(current_position, distance, target_node);

        assert_eq!(new_position_to_target, coord! { x: 5.0, y: 10.0 });
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
