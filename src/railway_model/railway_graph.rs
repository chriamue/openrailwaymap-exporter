use geo_types::{coord, Coord};
use petgraph::visit::IntoNodeReferences;
use petgraph::{stable_graph::NodeIndex, Graph, Undirected};
use std::collections::HashMap;

use super::{RailwayEdge, RailwayNode};

/// `RailwayGraph` represents a graph structure for railway networks.
///
/// The graph consists of nodes representing railway stations and junctions, and edges representing
/// the railway segments between the nodes. It also stores the node indices as a HashMap for easy
/// retrieval.
#[derive(Debug, Clone)]
pub struct RailwayGraph {
    /// The internal graph used to represent the railway network.
    ///
    /// The graph consists of `RailwayNode` instances as nodes and `RailwayEdge` instances as edges.
    /// It is an undirected graph.
    pub graph: Graph<RailwayNode, RailwayEdge, Undirected>,

    /// A HashMap that maps node IDs to their corresponding indices in the graph.
    ///
    /// This HashMap allows for quick and easy retrieval of node indices based on their IDs.
    pub node_indices: HashMap<i64, NodeIndex>,
}

impl PartialEq for RailwayGraph {
    fn eq(&self, other: &Self) -> bool {
        self.node_indices.eq(&other.node_indices)
    }
}

impl RailwayGraph {
    /// Calculate the bounding box of the graph.
    ///
    /// The bounding box is represented as a tuple containing the minimum and maximum
    /// latitude and longitude values of the nodes in the graph.
    ///
    /// # Returns
    ///
    /// A tuple containing two `Coordinate` structs representing the minimum and maximum coordinates
    /// of the bounding box of the graph.
    ///
    pub fn bounding_box(&self) -> (Coord, Coord) {
        let mut min_lat = std::f64::MAX;
        let mut min_lon = std::f64::MAX;
        let mut max_lat = std::f64::MIN;
        let mut max_lon = std::f64::MIN;

        for node in self.graph.node_references() {
            let lat = node.1.lat;
            let lon = node.1.lon;

            min_lat = min_lat.min(lat);
            min_lon = min_lon.min(lon);
            max_lat = max_lat.max(lat);
            max_lon = max_lon.max(lon);
        }

        (
            coord! { x: min_lon, y: min_lat},
            coord! { x: max_lon, y: max_lat},
        )
    }

    /// Calculate the total length of the railway network.
    ///
    /// The total length is the sum of the lengths of all edges in the graph.
    ///
    /// # Returns
    ///
    /// A `f64` value representing the total length of the railway network in meters.
    ///
    pub fn total_length(&self) -> f64 {
        self.graph
            .edge_references()
            .map(|edge| edge.weight().length)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use geo_types::coord;

    use crate::prelude::{
        from_railway_elements,
        overpass_api_client::{Coordinate, ElementType, RailwayElement},
    };
    use std::collections::HashMap;

    #[test]
    fn test_bounding_box() {
        let mut tags = HashMap::new();
        tags.insert("railway".to_string(), "station".to_string());

        let elements = vec![
            RailwayElement {
                id: 1,
                element_type: ElementType::Node,
                lat: Some(50.1109),
                lon: Some(8.6821),
                tags: Some(tags.clone()),
                nodes: None,
                geometry: None,
            },
            RailwayElement {
                id: 2,
                element_type: ElementType::Node,
                lat: Some(51.1109),
                lon: Some(9.6821),
                tags: Some(tags.clone()),
                nodes: None,
                geometry: None,
            },
            RailwayElement {
                id: 3,
                element_type: ElementType::Node,
                lat: Some(49.1109),
                lon: Some(7.6821),
                tags: Some(tags),
                nodes: None,
                geometry: None,
            },
        ];

        let railway_graph = from_railway_elements(&elements);
        let (min_coord, max_coord) = railway_graph.bounding_box();

        assert_eq!(
            min_coord,
            coord! {
                x: 7.6821,
                y: 49.1109,
            }
        );
        assert_eq!(
            max_coord,
            coord! {
                x: 9.6821,
                y: 51.1109,
            }
        );
    }

    #[test]
    fn test_total_length() {
        let elements = vec![
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
                lat: Some(50.1119),
                lon: Some(8.6831),
                tags: Some(HashMap::new()),
                nodes: None,
                geometry: None,
            },
            RailwayElement {
                id: 3,
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
                        lat: 50.1119,
                        lon: 8.6831,
                    },
                ]),
            },
        ];

        let railway_graph = from_railway_elements(&elements);
        let total_length = railway_graph.total_length();
        assert_eq!(total_length, 132.246);
    }
}
