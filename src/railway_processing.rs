use std::collections::{HashMap, HashSet};

use crate::{railway_element::ElementType, RailwayElement, RailwayNode};

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
/// use openrailwaymap_exporter::{RailwayElement, ElementType};
/// use openrailwaymap_exporter::create_nodes;
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

fn create_nodes_from_way_elements_without_existing(
    elements: &[RailwayElement],
    node_ids: &mut HashSet<i64>,
) -> Vec<RailwayNode> {
    let mut nodes: Vec<RailwayNode> = Vec::new();
    let node_id_to_element_ids = create_node_id_to_element_ids_map(elements);
    let id_to_element_map = create_id_to_element_map(elements);

    for (node_id, element_ids) in node_id_to_element_ids {
        if !node_ids.contains(&node_id) && element_ids.len() >= 2 {
            if let Some(element) = id_to_element_map.get(&element_ids[0]) {
                if let Some(geometry) = &element.geometry {
                    if let Some(first_coordinate) = geometry.first() {
                        let node = RailwayNode {
                            id: node_id,
                            lat: first_coordinate.lat,
                            lon: first_coordinate.lon,
                        };
                        nodes.push(node);
                        node_ids.insert(node_id);
                    }
                }
            }
        }
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Coordinate;
    use std::collections::HashMap;

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
