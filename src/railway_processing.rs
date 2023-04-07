use std::collections::HashSet;

use crate::{railway_element::ElementType, RailwayElement, RailwayNode};

pub fn create_nodes(elements: &[RailwayElement]) -> Vec<RailwayNode> {
    let mut nodes: Vec<RailwayNode> = Vec::new();
    let mut node_ids: HashSet<i64> = HashSet::new();

    for element in elements {
        match element.element_type {
            ElementType::Node => {
                if let (Some(lat), Some(lon)) = (element.lat, element.lon) {
                    if !node_ids.contains(&element.id) {
                        nodes.push(RailwayNode {
                            id: element.id,
                            lat,
                            lon,
                        });
                        node_ids.insert(element.id);
                    }
                }
            }
            ElementType::Way => {
                if let Some(element_nodes) = &element.nodes {
                    for i in 0..element_nodes.len() {
                        let node_id = element_nodes[i];

                        if !node_ids.contains(&node_id) {
                            if let Some(node) = find_node_by_id(elements, node_id) {
                                nodes.push(node);
                                node_ids.insert(node_id);
                            }
                        }
                    }
                }
            }
        }
    }

    nodes
}

pub fn find_node_by_id(elements: &[RailwayElement], id: i64) -> Option<RailwayNode> {
    elements
        .iter()
        .find(|element| element.element_type == ElementType::Node && element.id == id)
        .map(|element| RailwayNode {
            id: element.id,
            lat: element.lat.unwrap(),
            lon: element.lon.unwrap(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Coordinate;
    use std::collections::HashMap;

    #[test]
    fn test_find_node_by_id() {
        let elements = vec![
            RailwayElement::new_with_id(1),
            RailwayElement {
                element_type: ElementType::Node,
                id: 2,
                lat: Some(50.2291127),
                lon: Some(8.7190232),
                tags: None,
                nodes: None,
                geometry: None,
            },
            RailwayElement {
                element_type: ElementType::Way,
                id: 3,
                lat: None,
                lon: None,
                tags: None,
                nodes: Some(vec![1, 2]),
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

        let expected_node = RailwayNode {
            id: 2,
            lat: 50.2291127,
            lon: 8.7190232,
        };

        let result = find_node_by_id(&elements, 2);
        assert_eq!(result, Some(expected_node));

        let result = find_node_by_id(&elements, 3);
        assert_eq!(result, None);
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
}
