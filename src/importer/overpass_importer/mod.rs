//! `OverpassImporter` is a struct that implements the `RailwayGraphImporter` trait for importing
//! railway graph data from the Overpass API.
mod coordinate;
mod railway_element;
use crate::algorithms::Distance;
use crate::railway_model::{RailwayEdge, RailwayGraph, RailwayNode};
use anyhow::Result;
pub use coordinate::Coordinate;
use geo::{coord, Coord, LineString};
use geoutils::Location;
use petgraph::graph::Graph;
use petgraph::stable_graph::NodeIndex;
use petgraph::Undirected;
pub use railway_element::RailwayElement;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use uom::si::length::meter;

pub use self::railway_element::{count_node_elements, count_way_elements, ElementType};

use super::RailwayGraphImporter;

/// `OverpassImporter` is a struct that implements the `RailwayGraphImporter` trait for importing
/// railway graph data from the Overpass API.
pub struct OverpassImporter;

impl RailwayGraphImporter for OverpassImporter {
    fn import(input: &Value) -> Result<RailwayGraph> {
        let railway_elements = RailwayElement::from_json(input)?;
        Ok(from_railway_elements(&railway_elements))
    }
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
/// use openrailwaymap_exporter::importer::overpass_importer::{ElementType, RailwayElement, Coordinate};
/// use openrailwaymap_exporter::importer::overpass_importer::from_railway_elements;
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
/// let railway_graph = from_railway_elements(&elements);
/// println!("Created railway graph with {} nodes", railway_graph.graph.node_count());
/// ```
pub fn from_railway_elements(elements: &[RailwayElement]) -> RailwayGraph {
    let mut graph = Graph::<RailwayNode, RailwayEdge, Undirected>::new_undirected();
    let mut node_indices = HashMap::new();

    let nodes = create_nodes(elements);
    for node in &nodes {
        let node_index = graph.add_node(node.clone());
        node_indices.insert(node.id, node_index);
    }

    assert_eq!(nodes.len(), node_indices.len());

    for element in elements.iter() {
        if let ElementType::Way = element.element_type {
            if let (Some(nodes_ids), Some(geometry)) = (&element.nodes, &element.geometry) {
                let length = calculate_geometry_length(geometry);

                let (node_id, node_index) = find_next_existing_node(None, nodes_ids, &node_indices);
                let (next_node_id, next_node_index) =
                    find_next_existing_node(node_id, nodes_ids, &node_indices);

                if let (Some(node_index), Some(next_node_index)) = (node_index, next_node_index) {
                    assert_ne!(node_index, next_node_index);

                    let linestring: Vec<_> = {
                        let node1_coord: Coord<f64> =
                            coord! {x: graph[node_index].lon, y: graph[node_index].lat};
                        let node2_coord: Coord<f64> =
                            coord! {x: graph[next_node_index].lon, y: graph[next_node_index].lat};
                        let reverse = node1_coord
                            .distance(&coord! {x: geometry[0].lon, y: geometry[0].lat})
                            > node2_coord
                                .distance(&coord! {x: geometry[0].lon, y: geometry[0].lat});

                        if reverse {
                            geometry
                                .into_iter()
                                .rev()
                                .map(|coord| coord! { x: coord.lon, y: coord.lat })
                                .collect()
                        } else {
                            geometry
                                .iter()
                                .map(|coord| coord! { x: coord.lon, y: coord.lat })
                                .collect::<Vec<_>>()
                        }
                    };

                    graph.add_edge(
                        node_index,
                        next_node_index,
                        RailwayEdge {
                            id: element.id,
                            length,
                            path: LineString::from(linestring),
                            source: node_id.unwrap(),
                            target: next_node_id.unwrap(),
                        },
                    );
                }
            }
        }
    }
    RailwayGraph {
        graph,
        node_indices,
    }
}

/// Find the next existing node ID and its index in the `node_indices` HashMap after the given `start` ID.
///
/// This function searches the `node_ids` slice for the next existing node ID after the specified `start` ID.
/// If the next existing node ID is found, it returns a tuple `(Some(id), Some(index))`, where `id` is the found
/// node ID, and `index` is its index in the `node_indices` HashMap. If no existing node ID is found,
/// it returns `(None, None)`.
///
/// # Arguments
///
/// * `start` - An optional starting node ID to search from.
/// * `node_ids` - A reference to the slice containing the node IDs.
/// * `node_indices` - A reference to the HashMap containing the node indices.
///
/// # Returns
///
/// A tuple `(Option<i64>, Option<i64>)` containing the next existing node ID and its index if found, or `(None, None)` otherwise.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use petgraph::stable_graph::NodeIndex;
/// use openrailwaymap_exporter::importer::overpass_importer::find_next_existing_node;
///
/// let node_ids = vec![1, 3, 5];
/// let mut node_indices = HashMap::new();
/// node_indices.insert(1, NodeIndex::new(0));
/// node_indices.insert(3, NodeIndex::new(1));
/// node_indices.insert(5, NodeIndex::new(2));
///
/// assert_eq!(find_next_existing_node(Some(1), &node_ids, &node_indices), (Some(3), Some(NodeIndex::new(1))));
/// assert_eq!(find_next_existing_node(Some(3), &node_ids, &node_indices), (Some(5), Some(NodeIndex::new(2))));
/// assert_eq!(find_next_existing_node(Some(5), &node_ids, &node_indices), (None, None));
/// assert_eq!(find_next_existing_node(None, &node_ids, &node_indices), (Some(1), Some(NodeIndex::new(0))));
/// ```
pub fn find_next_existing_node(
    start: Option<i64>,
    node_ids: &[i64],
    node_indices: &HashMap<i64, NodeIndex>,
) -> (Option<i64>, Option<NodeIndex>) {
    let start_pos = start.and_then(|start_id| node_ids.iter().position(|&id| id == start_id));

    for (pos, &id) in node_ids.iter().enumerate() {
        if start_pos.map_or(true, |start_pos| pos > start_pos) {
            if let Some(index) = node_indices.get(&id) {
                return (Some(id), Some(*index));
            }
        }
    }
    (None, None)
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
/// use openrailwaymap_exporter::importer::overpass_importer::Coordinate;
/// use openrailwaymap_exporter::importer::overpass_importer::calculate_geometry_length;
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

/// Create a vector of `RailwayNode`s from the provided `RailwayElement`s.
///
/// This function combines the nodes created from `RailwayElement`s of type `Node` and
/// the nodes created from `RailwayElement`s of type `Way` which don't have a corresponding
/// node in the elements of type `Node`.
///
/// # Arguments
///
/// * `elements` - A slice of `RailwayElement`s from which the nodes will be created.
///
/// # Returns
///
/// A vector of `RailwayNode`s created from the input `RailwayElement`s.
///
/// # Example
///
/// ```
/// use openrailwaymap_exporter::importer::overpass_importer::create_nodes;
/// use openrailwaymap_exporter::importer::overpass_importer::{ElementType, RailwayElement};
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
/// let nodes = create_nodes(&elements);
/// println!("Created {} nodes", nodes.len());
/// assert_eq!(nodes.len(), 1);
/// ```
pub fn create_nodes(elements: &[RailwayElement]) -> Vec<RailwayNode> {
    let nodes = create_nodes_from_node_elements(elements);
    let mut node_ids: HashSet<i64> = node_ids_from_nodes(&nodes);
    let implicit_nodes = create_nodes_from_way_elements_without_existing(elements, &mut node_ids);
    [nodes, implicit_nodes].concat()
}

fn create_nodes_from_node_elements(elements: &[RailwayElement]) -> Vec<RailwayNode> {
    let mut nodes: Vec<RailwayNode> = Vec::new();

    for element in elements {
        if let ElementType::Node = element.element_type {
            if let (Some(lat), Some(lon)) = (element.lat, element.lon) {
                let node = RailwayNode {
                    id: element.id,
                    lat,
                    lon,
                };
                nodes.push(node);
            }
        }
    }
    nodes
}

fn node_ids_from_nodes(nodes: &[RailwayNode]) -> HashSet<i64> {
    nodes.iter().map(|node| node.id).collect()
}

fn create_node_id_to_element_ids_map(elements: &[RailwayElement]) -> HashMap<i64, Vec<i64>> {
    let mut node_id_to_element_ids: HashMap<i64, Vec<i64>> = HashMap::new();

    for element in elements {
        if let ElementType::Way = element.element_type {
            if let Some(element_nodes) = &element.nodes {
                for node_id in element_nodes {
                    node_id_to_element_ids
                        .entry(*node_id)
                        .or_insert_with(Vec::new)
                        .push(element.id);
                }
            }
        }
    }

    node_id_to_element_ids
}

fn create_id_to_element_map<'a>(
    elements: &'a [RailwayElement],
) -> HashMap<i64, &'a RailwayElement> {
    let mut id_to_element_map: HashMap<i64, &'a RailwayElement> = HashMap::new();

    for element in elements {
        id_to_element_map.insert(element.id, element);
    }

    id_to_element_map
}
/// Create railway nodes from two railway elements that share a node ID but do not have an existing
/// common node. The nodes are created by finding the two coordinates in the geometries of the two
/// railway elements that are closest to each other.
///
/// # Arguments
///
/// * `elements` - A slice of railway elements to process.
/// * `node_ids` - A mutable reference to a HashSet of node IDs to be updated with newly created nodes.
///
/// # Returns
///
/// A vector of railway nodes created from the input railway elements.
pub fn create_nodes_from_way_elements_without_existing(
    elements: &[RailwayElement],
    node_ids: &mut HashSet<i64>,
) -> Vec<RailwayNode> {
    let node_id_to_element_ids = create_node_id_to_element_ids_map(elements);
    let id_to_element_map = create_id_to_element_map(elements);

    let mut new_nodes = Vec::new();

    for (node_id, element_ids) in node_id_to_element_ids {
        if element_ids.len() == 2 && !node_ids.contains(&node_id) {
            let element1 = id_to_element_map.get(&element_ids[0]);
            let element2 = id_to_element_map.get(&element_ids[1]);

            if let (Some(element1), Some(element2)) = (element1, element2) {
                if let (Some(geometry1), Some(geometry2)) = (&element1.geometry, &element2.geometry)
                {
                    let mut min_distance = f64::MAX;
                    let mut closest_coords = (
                        coord! {x: geometry1[0].lon, y: geometry1[0].lat},
                        coord! {x: geometry2[0].lon, y: geometry2[0].lat },
                    );

                    for coord1 in geometry1 {
                        let coord1_geo = coord! { x: coord1.lon, y: coord1.lat };
                        for coord2 in geometry2 {
                            let coord2_geo = coord! { x: coord2.lon, y: coord2.lat };
                            let distance = coord1_geo.distance(&coord2_geo).get::<meter>();
                            if distance < min_distance {
                                min_distance = distance;
                                closest_coords = (coord1_geo.clone(), coord2_geo.clone());
                            }
                        }
                    }

                    let node = RailwayNode {
                        id: node_id,
                        lat: closest_coords.0.y,
                        lon: closest_coords.0.x,
                    };

                    new_nodes.push(node);
                    node_ids.insert(node_id);
                }
            }
        }
    }

    new_nodes
}

#[cfg(test)]
mod tests {

    use serde_json::json;

    use crate::importer::overpass_importer::railway_element::ElementType;

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
                    Coordinate { lat: 0.0, lon: 3.5 },
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

        let railway_graph = from_railway_elements(&elements);
        assert_eq!(railway_graph.graph.node_count(), 3);

        let node_index_1 = railway_graph.node_indices.get(&1).unwrap();
        let node_1 = &railway_graph.graph[*node_index_1];
        assert_eq!(node_1.lat, 0.0);
        assert_eq!(node_1.lon, 1.0);

        let node_index_3 = railway_graph.node_indices.get(&3).unwrap();
        let node_3 = &railway_graph.graph[*node_index_3];
        assert_eq!(node_3.lat, 0.0);
        assert_eq!(node_3.lon, 3.5);
    }

    #[test]
    fn test_importer() {
        let json_value = json!({
            "elements": [
                {
                    "type": "node",
                    "id": 1,
                    "lat": 50.1191127,
                    "lon": 8.6090232,
                    "tags": {
                        "railway": "switch",
                        "railway:switch": "default",
                        "railway:turnout_side": "right"
                    }
                },
                {
                    "type": "way",
                    "id": 2,
                    "nodes": [1, 2, 3],
                    "tags": {
                        "railway": "rail"
                    }
                }
            ]
        });

        let railway_graph = OverpassImporter::import(&json_value).unwrap();
        assert_eq!(railway_graph.graph.node_count(), 1);

        let node_index_1 = railway_graph.node_indices.get(&1).unwrap();
        let node_1 = &railway_graph.graph[*node_index_1];
        assert_eq!(node_1.lat, 50.1191127);
        assert_eq!(node_1.lon, 8.6090232);
    }

    #[test]
    fn test_find_next_existing_node() {
        let node_ids = vec![1, 3, 5];
        let mut node_indices = HashMap::new();
        node_indices.insert(1, NodeIndex::new(0));
        node_indices.insert(3, NodeIndex::new(1));
        node_indices.insert(5, NodeIndex::new(2));

        assert_eq!(
            find_next_existing_node(Some(1), &node_ids, &node_indices),
            (Some(3), Some(NodeIndex::new(1)))
        );
        assert_eq!(
            find_next_existing_node(Some(3), &node_ids, &node_indices),
            (Some(5), Some(NodeIndex::new(2)))
        );
        assert_eq!(
            find_next_existing_node(Some(5), &node_ids, &node_indices),
            (None, None)
        );
        assert_eq!(
            find_next_existing_node(None, &node_ids, &node_indices),
            (Some(1), Some(NodeIndex::new(0)))
        );
    }

    #[test]
    fn test_create_nodes() {
        let elements = vec![
            RailwayElement {
                id: 1,
                element_type: ElementType::Node,
                lat: Some(50.1191127),
                lon: Some(8.6090232),
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
                nodes: Some(vec![1, 3]),
                geometry: None,
            },
            RailwayElement {
                id: 3,
                element_type: ElementType::Node,
                lat: Some(50.1191177),
                lon: Some(8.6090237),
                tags: Some(HashMap::new()),
                nodes: None,
                geometry: None,
            },
        ];

        let nodes = create_nodes(&elements);
        assert_eq!(nodes.len(), 2);

        let node_a = &nodes[0];
        assert_eq!(node_a.id, 1);
        assert_eq!(node_a.lat, 50.1191127);
        assert_eq!(node_a.lon, 8.6090232);

        let node_b = &nodes[1];
        assert_eq!(node_b.id, 3);
        assert_eq!(node_b.lat, 50.1191177);
        assert_eq!(node_b.lon, 8.6090237);
    }

    #[test]
    fn test_create_nodes_from_way_elements_without_existing() {
        let elements = vec![
            RailwayElement {
                id: 2,
                element_type: ElementType::Way,
                lat: None,
                lon: None,
                tags: Some(HashMap::new()),
                nodes: Some(vec![1, 3]),
                geometry: Some(vec![
                    Coordinate {
                        lat: 50.1191127,
                        lon: 8.6090232,
                    },
                    Coordinate {
                        lat: 50.2291127,
                        lon: 8.7190232,
                    },
                ]),
            },
            RailwayElement {
                id: 3,
                element_type: ElementType::Way,
                lat: None,
                lon: None,
                tags: Some(HashMap::new()),
                nodes: Some(vec![3, 4]),
                geometry: Some(vec![
                    Coordinate {
                        lat: 50.1191127,
                        lon: 8.6090232,
                    },
                    Coordinate {
                        lat: 50.2291127,
                        lon: 8.7190232,
                    },
                ]),
            },
        ];

        let mut node_ids = HashSet::new();
        node_ids.insert(1);

        let nodes = create_nodes_from_way_elements_without_existing(&elements, &mut node_ids);
        assert_eq!(nodes.len(), 1);

        let node_a = &nodes[0];
        assert_eq!(node_a.id, 3);
        assert_eq!(node_a.lat, 50.1191127);
        assert_eq!(node_a.lon, 8.6090232);
    }

    #[test]
    fn test_create_nodes_implicit() {
        let elements = vec![
            RailwayElement {
                id: 1,
                element_type: ElementType::Way,
                lat: None,
                lon: None,
                tags: Some(HashMap::new()),
                nodes: Some(vec![3, 4]),
                geometry: Some(vec![Coordinate {
                    lat: 50.1191127,
                    lon: 8.6090232,
                }]),
            },
            RailwayElement {
                id: 2,
                element_type: ElementType::Way,
                lat: None,
                lon: None,
                tags: Some(HashMap::new()),
                nodes: Some(vec![4, 5]),
                geometry: Some(vec![Coordinate {
                    lat: 50.1191127,
                    lon: 8.6090232,
                }]),
            },
        ];

        let railway_nodes = create_nodes(&elements);
        assert_eq!(railway_nodes.len(), 1);
    }

    #[test]
    fn test_vilbel_json() {
        use crate::tests::test_json_vilbel;

        let railway_elements = RailwayElement::from_json(&test_json_vilbel()).unwrap();

        let nodes = create_nodes_from_node_elements(&railway_elements);
        assert_eq!(nodes.len(), 20);
    }
}
