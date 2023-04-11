use crate::railway_model::RailwayGraph;
use geoutils::Location;
use petgraph::{
    algo::{astar, dijkstra},
    stable_graph::NodeIndex,
};
use std::borrow::Borrow;

/// `PathFinding` trait provides pathfinding algorithms for railway networks.
pub trait PathFinding {
    /// Calculate the shortest path distance between two nodes.
    ///
    /// # Arguments
    /// * `source` - The ID of the source node.
    /// * `target` - The ID of the target node.
    ///
    /// # Returns
    /// Returns the distance of the shortest path between the source and target nodes if it exists.
    fn shortest_path_distance(&self, source: i64, target: i64) -> Option<f64>;

    /// Calculate the shortest path between two nodes as a list of node IDs.
    ///
    /// # Arguments
    /// * `start` - The ID of the start node.
    /// * `end` - The ID of the end node.
    ///
    /// # Returns
    /// Returns a `Vec<i64>` containing the IDs of the nodes in the shortest path if it exists.
    /// The returned vector includes the start and end node IDs.
    fn shortest_path_nodes(&self, start: i64, end: i64) -> Option<Vec<i64>>;

    /// Calculate the shortest path between two nodes as a list of edge IDs.
    ///
    /// # Arguments
    /// * `start` - The ID of the start node.
    /// * `end` - The ID of the end node.
    ///
    /// # Returns
    /// Returns a `Vec<i64>` containing the IDs of the edges in the shortest path if it exists.
    fn shortest_path_edges(&self, start: i64, end: i64) -> Option<Vec<i64>>;
}

impl PathFinding for RailwayGraph {
    fn shortest_path_distance(&self, source: i64, target: i64) -> Option<f64> {
        let source_index = self.node_indices.get(&source)?;
        let target_index = self.node_indices.get(&target)?;

        let shortest_path = dijkstra(&self.graph, *source_index, Some(*target_index), |e| {
            e.weight().length
        });

        shortest_path.get(target_index).copied()
    }

    fn shortest_path_nodes(&self, start: i64, end: i64) -> Option<Vec<i64>> {
        let start_index = self.node_indices.get(&start)?;
        let end_index = self.node_indices.get(&end)?;

        let heuristic = |index: NodeIndex| -> f64 {
            let lat1 = self.graph[index].lat;
            let lon1 = self.graph[index].lon;
            let lat2 = self.graph[*end_index].lat;
            let lon2 = self.graph[*end_index].lon;

            Location::new(lat1, lon1)
                .distance_to(&Location::new(lat2, lon2))
                .unwrap()
                .meters()
        };

        let path = astar(
            &self.graph,
            *start_index,
            |idx| idx == *end_index,
            |e| *e.weight().length.borrow(),
            heuristic,
        );

        path.map(|(_, path_indices)| {
            path_indices
                .into_iter()
                .map(|idx| self.graph[idx].id)
                .collect::<Vec<i64>>()
        })
    }

    fn shortest_path_edges(&self, start: i64, end: i64) -> Option<Vec<i64>> {
        let node_path = self.shortest_path_nodes(start, end)?;
        if node_path.len() < 2 {
            return None;
        }

        node_path
            .windows(2)
            .filter_map(|pair| {
                let start_node_index = *self.node_indices.get(&pair[0])?;
                let end_node_index = *self.node_indices.get(&pair[1])?;
                let edge_index = self.graph.find_edge(start_node_index, end_node_index)?;
                Some(self.graph[edge_index].id)
            })
            .collect::<Vec<i64>>()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::{
        importer::overpass_importer::{Coordinate, ElementType, RailwayElement},
        prelude::from_railway_elements,
    };

    use super::*;
    use std::collections::HashMap;

    fn test_elements() -> Vec<RailwayElement> {
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
    fn test_shortest_path_distance() {
        let railway_graph = from_railway_elements(&test_elements());

        assert_relative_eq!(
            railway_graph.shortest_path_distance(1, 2).unwrap(),
            1322.421,
            epsilon = 0.1
        );
        assert_relative_eq!(
            railway_graph.shortest_path_distance(1, 3).unwrap(),
            3134.2,
            epsilon = 0.1
        );
        assert_relative_eq!(
            railway_graph.shortest_path_distance(2, 3).unwrap(),
            1811.801
        );
        assert_eq!(railway_graph.shortest_path_distance(1, 4), None);
    }

    #[test]
    fn test_shortest_path_nodes() {
        let railway_graph = from_railway_elements(&test_elements());

        assert_eq!(railway_graph.shortest_path_nodes(1, 2), Some(vec![1, 2]));
        assert_eq!(railway_graph.shortest_path_nodes(1, 3), Some(vec![1, 2, 3]));
        assert_eq!(railway_graph.shortest_path_nodes(2, 3), Some(vec![2, 3]));
        assert_eq!(railway_graph.shortest_path_nodes(1, 4), None);
    }

    #[test]
    fn test_shortest_path_edges() {
        let railway_graph = from_railway_elements(&test_elements());

        assert_eq!(railway_graph.shortest_path_edges(1, 2), Some(vec![4]));
        assert_eq!(railway_graph.shortest_path_edges(1, 3), Some(vec![4, 5]));
        assert_eq!(railway_graph.shortest_path_edges(2, 3), Some(vec![5]));
        assert_eq!(railway_graph.shortest_path_edges(1, 4), None);
    }
}
