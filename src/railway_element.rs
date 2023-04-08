use crate::Coordinate;
use serde::{ser::Error, Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Represents the type of a `RailwayElement`.
///
/// `ElementType` is an enumeration with two possible values: `Way` and `Node`.
/// It is used to represent the type of an element in a railway network.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ElementType {
    /// Represents a `Way` element in the railway network.
    ///
    /// A `Way` element is a linear feature, such as a railway track or a route.
    /// It consists of an ordered list of nodes that define the geometry of the way.
    Way,

    /// Represents a `Node` element in the railway network.
    ///
    /// A `Node` element is a point feature, such as a railway station or a junction.
    /// It is defined by its latitude and longitude coordinates.
    Node,
}

/// Represents an element of a railway network.
///
/// A `RailwayElement` struct contains information about a railway element, such as its ID, type, tags, nodes,
/// geometry, latitude, and longitude. It can be used to store and manipulate railway data.
///
/// # Example
///
/// ```
/// use openrailwaymap_exporter::{RailwayElement, ElementType};
///
/// let element = RailwayElement::new_with_id(1);
/// assert_eq!(element.id, 1);
/// assert_eq!(element.element_type, ElementType::Node);
/// ```
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct RailwayElement {
    /// The unique identifier of the railway element.
    pub id: i64,
    /// Optional key-value pairs associated with the railway element.
    pub tags: Option<HashMap<String, String>>,
    /// The type of the railway element, either `Way` or `Node`.
    #[serde(rename = "type")]
    pub element_type: ElementType,
    /// An optional ordered list of node IDs that define the geometry of a `Way` element.
    pub nodes: Option<Vec<i64>>,
    /// An optional list of coordinates that represent the geometry of a `Way` element.
    pub geometry: Option<Vec<Coordinate>>,
    /// The latitude coordinate of a `Node` element.
    pub lat: Option<f64>,
    /// The longitude coordinate of a `Node` element.
    pub lon: Option<f64>,
}

impl RailwayElement {
    /// Deserialize a JSON value into a vector of `RailwayElement` instances.
    ///
    /// # Arguments
    ///
    /// * `json_value` - A reference to a JSON value containing railway elements data.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `RailwayElement` instances on success, or a `serde_json::Error` on failure.
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
    /// Create a new `RailwayElement` instance with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the new `RailwayElement` instance.
    ///
    /// # Returns
    ///
    /// A new `RailwayElement` instance with the specified ID.
    pub fn new_with_id(id: i64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

/// Counts the number of `Way` elements in a vector of `RailwayElement`s.
///
/// This function takes a slice of `RailwayElement`s as input and returns the count of `Way` elements as a `usize`.
///
/// # Arguments
///
/// * `elements` - A slice of `RailwayElement`s to count the `Way` elements in.
///
/// # Example
///
/// ```
/// use openrailwaymap_exporter::{ElementType, RailwayElement};
/// use openrailwaymap_exporter::count_way_elements;
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
/// let way_count = count_way_elements(&elements);
/// assert_eq!(way_count, 1);
/// ```
pub fn count_way_elements(elements: &[RailwayElement]) -> usize {
    elements
        .iter()
        .filter(|element| element.element_type == ElementType::Way)
        .count()
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

    #[test]
    fn test_count_way_elements() {
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
                element_type: ElementType::Way,
                lat: None,
                lon: None,
                tags: Some(HashMap::new()),
                nodes: Some(vec![1, 3]),
                geometry: None,
            },
            RailwayElement {
                id: 3,
                element_type: ElementType::Way,
                lat: None,
                lon: None,
                tags: Some(HashMap::new()),
                nodes: Some(vec![3, 5]),
                geometry: None,
            },
        ];

        let way_count = count_way_elements(&elements);
        assert_eq!(way_count, 2);
    }
}
