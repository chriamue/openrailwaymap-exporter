use geoutils::Location;
use petgraph::visit::IntoNodeReferences;
use petgraph::{stable_graph::NodeIndex, Graph, Undirected};
use std::collections::HashMap;

use crate::railway_element::{ElementType, RailwayElement};
use crate::railway_processing::create_nodes;
use crate::Coordinate;

use super::{RailwayEdge, RailwayNode};

/// `RailwayGraph` represents a graph structure for railway networks.
///
/// The graph consists of nodes representing railway stations and junctions, and edges representing
/// the railway segments between the nodes. It also stores the node indices as a HashMap for easy
/// retrieval.
#[derive(Debug)]
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
    pub fn bounding_box(&self) -> (Coordinate, Coordinate) {
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
            Coordinate {
                lat: min_lat,
                lon: min_lon,
            },
            Coordinate {
                lat: max_lat,
                lon: max_lon,
            },
        )
    }

    /// Create a `RailwayGraph` from a vector of `RailwayElement`s.
    ///
    /// The function processes the input elements to create a graph with nodes and edges.
    ///
    /// # Arguments
    ///
    /// * `elements` - A vector of `RailwayElement`s from which the graph will be created.
    ///
    /// # Returns
    ///
    /// A `RailwayGraph` created from the input `RailwayElement`s.
    ///
    /// # Example
    ///
    /// ```
    /// use openrailwaymap_exporter::{ElementType, RailwayElement};
    /// use openrailwaymap_exporter::Coordinate;
    /// use openrailwaymap_exporter::RailwayGraph;
    /// use std::collections::HashMap;
    ///
    /// let elements = vec![
    ///     RailwayElement {
    ///         id: 1,
    ///         element_type: ElementType::Node,
    ///         lat: Some(50.1109),
    ///         lon: Some(8.6821),
    ///         tags: Some(HashMap::new()),
    ///         nodes: None,
    ///         geometry: None,
    ///     },
    ///     RailwayElement {
    ///         id: 2,
    ///         element_type: ElementType::Way,
    ///         lat: None,
    ///         lon: None,
    ///         tags: Some(HashMap::new()),
    ///         nodes: Some(vec![1, 3]),
    ///         geometry: None,
    ///     },
    /// ];
    ///
    /// let railway_graph = RailwayGraph::from_railway_elements(&elements);
    /// println!("Created railway graph with {} nodes", railway_graph.graph.node_count());
    /// ```
    pub fn from_railway_elements(elements: &[RailwayElement]) -> Self {
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

        let connections = find_connected_elements(elements);
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
            for elem_b in elements.iter().skip(i + 1) {
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

        let railway_graph = RailwayGraph::from_railway_elements(&elements);
        let (min_coord, max_coord) = railway_graph.bounding_box();

        assert_eq!(
            min_coord,
            Coordinate {
                lat: 49.1109,
                lon: 7.6821
            }
        );
        assert_eq!(
            max_coord,
            Coordinate {
                lat: 51.1109,
                lon: 9.6821
            }
        );
    }

    #[test]
    fn test_from_railway_elements() {
        let elements = vec![
            RailwayElement {
                id: 1,
                element_type: ElementType::Node,
                lat: Some(0.0),
                lon: Some(1.0),
                tags: Some(HashMap::new()),
                nodes: None,
                geometry: None,
            },
            RailwayElement {
                id: 2,
                element_type: ElementType::Way,
                lat: None,
                lon: None,
                tags: Some(HashMap::new()),
                nodes: Some(vec![1, 7, 3, 8]),
                geometry: Some(vec![
                    Coordinate { lat: 0.0, lon: 0.0 },
                    Coordinate { lat: 0.0, lon: 3.0 },
                ]),
            },
            RailwayElement {
                id: 5,
                element_type: ElementType::Node,
                lat: Some(0.0),
                lon: Some(5.0),
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
                nodes: Some(vec![9, 3, 10, 5]),
                geometry: Some(vec![
                    Coordinate { lat: 0.0, lon: 3.0 },
                    Coordinate { lat: 0.0, lon: 5.0 },
                ]),
            },
        ];

        let railway_graph = RailwayGraph::from_railway_elements(&elements);
        assert_eq!(railway_graph.graph.node_count(), 3);

        let node_index_1 = railway_graph.node_indices.get(&1).unwrap();
        let node_1 = &railway_graph.graph[*node_index_1];
        assert_eq!(node_1.lat, 0.0);
        assert_eq!(node_1.lon, 1.0);

        let node_index_3 = railway_graph.node_indices.get(&3).unwrap();
        let node_3 = &railway_graph.graph[*node_index_3];
        assert_eq!(node_3.lat, 0.0);
        assert_eq!(node_3.lon, 3.0);
    }
}
