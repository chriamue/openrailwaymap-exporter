use geo::{coord, Coord};
use petgraph::visit::IntoNodeReferences;
use transit_grid::prelude::TransitNetwork;

use crate::types::{EdgeId, NodeId};

use super::{RailwayEdge, RailwayNode};

/// A RailwayGraph is a TransitNetwork with RailwayNode and RailwayEdge as node and edge types.
pub type RailwayGraph = TransitNetwork<Coord, f64>;

/// An extension trait for the RailwayGraph.
pub trait RailwayGraphExt {
    /// Retrieve an edge from the graph by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the edge to be retrieved.
    ///
    /// # Returns
    ///
    /// An `Option<RailwayEdge>` that contains the edge if found, or `None` if not found.
    ///
    fn get_edge_by_id(&self, id: EdgeId) -> Option<RailwayEdge>;

    /// Returns a reference to a RailwayNode with the specified NodeId if it exists in the graph.
    ///
    /// This method searches the railway graph for a node with the given NodeId. If the node is found,
    /// it returns a reference to the RailwayNode. If the node is not found, it returns None.
    ///
    /// # Arguments
    ///
    /// * id - The NodeId of the node to be retrieved from the railway graph.
    ///
    /// # Returns
    ///
    /// An Option containing a reference to the RailwayNode if it exists, otherwise None.
    fn get_node_by_id(&self, id: NodeId) -> Option<&RailwayNode>;

    /// Retrieve the railway edge between two nodes.
    ///
    /// # Arguments
    ///
    /// * `start_node_id` - The ID of the starting node.
    /// * `end_node_id` - The ID of the ending node.
    ///
    /// # Returns
    ///
    /// An `Option<&RailwayEdge>` that contains the railway edge connecting the two nodes if it exists,
    /// or `None` if no such edge exists.
    ///
    fn railway_edge(&self, start_node_id: NodeId, end_node_id: NodeId) -> Option<&RailwayEdge>;
    /// Retrieve the edges connected to a node by its ID.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The ID of the node whose edges are to be retrieved.
    ///
    /// # Returns
    ///
    /// A `Vec<&RailwayEdge>` containing the edges connected to the node, or an empty vector if the node is not found.
    ///
    fn get_edges_of_node(&self, node_id: NodeId) -> Vec<&RailwayEdge>;
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
    fn bounding_box(&self) -> (Coord, Coord);
    /// Calculate the total length of the railway network.
    ///
    /// The total length is the sum of the lengths of all edges in the graph.
    ///
    /// # Returns
    ///
    /// A `f64` value representing the total length of the railway network in meters.
    ///
    fn total_length(&self) -> f64;
    /// Returns the nearest node to the given position on the specified edge.
    ///
    /// # Arguments
    ///
    /// * `edge_id` - The ID of the edge.
    /// * `position_on_edge` - The position on the edge, ranging from 0.0 to 1.0.
    /// * `current_node_id` - An optional `NodeId` of the current node to determine the start node.
    ///
    /// # Returns
    ///
    /// An `Option<NodeId>` containing the ID of the nearest node if found, or `None` if the edge is not found.
    fn nearest_node(
        &self,
        edge_id: EdgeId,
        position_on_edge: f64,
        current_node_id: Option<NodeId>,
    ) -> Option<NodeId>;
}

impl RailwayGraphExt for RailwayGraph {
    fn get_edge_by_id(&self, id: EdgeId) -> Option<RailwayEdge> {
        for edge in self.physical_graph.graph.edge_references() {
            if edge.weight().id == id {
                return Some(edge.weight().clone());
            }
        }
        None
    }

    fn get_node_by_id(&self, id: NodeId) -> Option<&RailwayNode> {
        let node_index = self.physical_graph.id_to_index(id);
        if let Some(node_index) = node_index {
            return Some(&self.physical_graph.graph[*node_index]);
        }
        None
    }

    fn railway_edge(&self, start_node_id: NodeId, end_node_id: NodeId) -> Option<&RailwayEdge> {
        self.physical_graph
            .get_transit_edge(start_node_id, end_node_id)
    }

    fn get_edges_of_node(&self, node_id: NodeId) -> Vec<&RailwayEdge> {
        let node_index = self.physical_graph.id_to_index(node_id);
        if let Some(&node_index) = node_index {
            return self
                .physical_graph
                .graph
                .edges(node_index)
                .map(|e| e.weight())
                .collect();
        } else {
            Vec::new()
        }
    }

    fn bounding_box(&self) -> (Coord, Coord) {
        let mut min_lat = std::f64::MAX;
        let mut min_lon = std::f64::MAX;
        let mut max_lat = std::f64::MIN;
        let mut max_lon = std::f64::MIN;

        for node in self.physical_graph.graph.node_references() {
            let lat = node.1.location.y;
            let lon = node.1.location.x;

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

    fn total_length(&self) -> f64 {
        self.physical_graph
            .graph
            .edge_references()
            .map(|edge| edge.weight().length)
            .sum()
    }

    fn nearest_node(
        &self,
        edge_id: EdgeId,
        position_on_edge: f64,
        current_node_id: Option<NodeId>,
    ) -> Option<NodeId> {
        // Find the edge indices in the petgraph
        let mut edge_indices = self.physical_graph.graph.edge_indices();
        let edge_index = edge_indices.find(|idx| self.physical_graph.graph[*idx].id == edge_id)?;

        // Get the start and end nodes of the edge
        let (mut start_node_index, mut end_node_index) =
            self.physical_graph.graph.edge_endpoints(edge_index)?;

        if let Some(current_node_id) = current_node_id {
            if current_node_id == self.physical_graph.graph[end_node_index].id {
                std::mem::swap(&mut start_node_index, &mut end_node_index);
            }
        }

        let start_node = &self.physical_graph.graph[start_node_index];
        let end_node = &self.physical_graph.graph[end_node_index];

        // Clamp position_on_edge to the range [0.0, 1.0].
        let position_on_edge = position_on_edge.max(0.0).min(1.0);

        // Calculate the coordinates of the point on the edge.
        let start_coord = start_node.location;
        let end_coord = end_node.location;
        let point_on_edge = start_coord + (end_coord - start_coord) * position_on_edge;

        // Find the nearest node to the point on the edge.
        let mut nearest_node_index = None;
        let mut nearest_distance = f64::MAX;
        for node_index in self.physical_graph.graph.node_indices() {
            let node = &self.physical_graph.graph[node_index];
            let coord = node.location;
            let distance = point_distance(&point_on_edge, &coord);

            if distance < nearest_distance {
                nearest_node_index = Some(node_index);
                nearest_distance = distance;
            }
        }

        nearest_node_index.map(|index| self.physical_graph.graph[index].id)
    }
}

fn point_distance(coord1: &Coord, coord2: &Coord) -> f64 {
    let dx = coord1.x - coord2.x;
    let dy = coord1.y - coord2.y;
    (dx * dx + dy * dy).sqrt()
}

#[cfg(test)]
mod tests {
    use crate::{
        importer::overpass_importer::{
            from_railway_elements, Coordinate, ElementType, RailwayElement,
        },
        prelude::RailwayGraphExt,
    };
    use geo::coord;
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

    #[test]
    fn test_railway_edge() {
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
                tags: Some(tags.clone()),
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
                        lat: 51.1109,
                        lon: 9.6821,
                    },
                ]),
            },
        ];

        let railway_graph = from_railway_elements(&elements);

        println!("{:?}", railway_graph.physical_graph.graph);

        // Test for a valid edge.
        let edge = railway_graph.railway_edge(1, 2);
        assert!(edge.is_some());
        assert_eq!(edge.unwrap().id, 4);

        // Test for an invalid edge.
        let edge = railway_graph.railway_edge(1, 3);
        assert!(edge.is_none());
    }

    #[test]
    fn test_get_edges_of_node() {
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
                tags: Some(tags.clone()),
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
                        lat: 51.1109,
                        lon: 9.6821,
                    },
                ]),
            },
        ];

        let railway_graph = from_railway_elements(&elements);

        // Test for a node with edges.
        let edges = railway_graph.get_edges_of_node(1);
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].id, 4);

        // Test for a node without edges.
        let edges = railway_graph.get_edges_of_node(3);
        assert_eq!(edges.len(), 0);

        // Test for a non-existent node.
        let edges = railway_graph.get_edges_of_node(999);
        assert_eq!(edges.len(), 0);
    }

    #[test]
    fn test_nearest_node() {
        // Create a simple RailwayGraph with three nodes and two edges
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

        // Call the nearest_node function
        let edge_id = 3;
        let position_on_edge = 0.52;
        let nearest_node_id = railway_graph.nearest_node(edge_id, position_on_edge, Some(1));

        assert_eq!(nearest_node_id, Some(2));

        let nearest_node_id = railway_graph.nearest_node(edge_id, position_on_edge, Some(2));
        assert_eq!(nearest_node_id, Some(1));

        let nearest_node_id = railway_graph.nearest_node(5, position_on_edge, Some(2));
        assert_eq!(nearest_node_id, None);
    }
}
