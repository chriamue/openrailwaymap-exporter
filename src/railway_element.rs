use crate::Coordinate;
use serde::{ser::Error, Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ElementType {
    Way,
    Node,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct RailwayElement {
    pub id: i64,
    pub tags: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub element_type: ElementType,
    pub nodes: Option<Vec<i64>>,
    pub geometry: Option<Vec<Coordinate>>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

impl RailwayElement {
    pub fn from_json(json_value: &Value) -> Result<Vec<RailwayElement>, serde_json::Error> {
        let railway_elements = json_value["elements"]
            .as_array()
            .ok_or_else(|| serde_json::Error::custom("Elements parsing error"))?
            .iter()
            .filter_map(|elem| serde_json::from_value::<RailwayElement>(elem.clone()).ok())
            .collect::<Vec<RailwayElement>>();
        Ok(railway_elements)
    }
}

impl Default for RailwayElement {
    fn default() -> Self {
        Self {
            id: 0,
            tags: None,
            element_type: ElementType::Node,
            nodes: None,
            geometry: None,
            lat: None,
            lon: None,
        }
    }
}

impl RailwayElement {
    pub fn new_with_id(id: i64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_railway_element_default() {
        let default_element = RailwayElement::default();
        assert_eq!(default_element.id, 0);
        assert!(default_element.tags.is_none());
        assert_eq!(default_element.element_type, ElementType::Node);
        assert!(default_element.nodes.is_none());
        assert!(default_element.geometry.is_none());
        assert!(default_element.lat.is_none());
        assert!(default_element.lon.is_none());
    }

    #[test]
    fn test_railway_element_new_with_id() {
        let id = 42;
        let element_with_id = RailwayElement::new_with_id(id);
        assert_eq!(element_with_id.id, id);
        assert!(element_with_id.tags.is_none());
        assert_eq!(element_with_id.element_type, ElementType::Node);
        assert!(element_with_id.nodes.is_none());
        assert!(element_with_id.geometry.is_none());
        assert!(element_with_id.lat.is_none());
        assert!(element_with_id.lon.is_none());
    }

    #[test]
    fn test_railway_element_from_json() {
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

        let result = RailwayElement::from_json(&json_value);
        assert!(result.is_ok());

        let elements = result.unwrap();
        assert_eq!(elements.len(), 2);

        let node_element = &elements[0];
        assert_eq!(node_element.id, 1);
        assert_eq!(node_element.element_type, ElementType::Node);
        assert_eq!(node_element.lat, Some(50.1191127));
        assert_eq!(node_element.lon, Some(8.6090232));

        let way_element = &elements[1];
        assert_eq!(way_element.id, 2);
        assert_eq!(way_element.element_type, ElementType::Way);
        assert_eq!(way_element.nodes, Some(vec![1, 2, 3]));
    }
}
