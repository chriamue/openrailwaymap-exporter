//! Export module for the OpenRailwayMap Exporter.
//!
//! This module provides functions to generate a DOT or SVG string representation of a RailwayGraph.
//! The generated strings can be used to visualize the railway infrastructure data.

pub use crate::exporter::svg::generate_svg_string;
use crate::prelude::RailwayGraph;
use petgraph::dot::{Config, Dot};
use std::error::Error;

/// Generates a DOT string representation of a given RailwayGraph.
///
/// The DOT string can be used to visualize the graph using tools like Graphviz.
///
/// # Arguments
///
/// * `graph` - A reference to a RailwayGraph.
///
/// # Returns
///
/// A `Result` containing a DOT-formatted `String` on success, or a `Box<dyn Error>` on failure.
///
/// # Example
///
/// ```
/// use openrailwaymap_exporter::importer::overpass_importer::RailwayElement;
/// use openrailwaymap_exporter::prelude::generate_dot_string;
/// use openrailwaymap_exporter::importer::overpass_importer::from_railway_elements;
///
/// let elements = vec![
///     RailwayElement::new_with_id(1),
///     RailwayElement::new_with_id(2),
/// ];
///
/// let railway_graph = from_railway_elements(&elements);
/// let dot_string = generate_dot_string(&railway_graph).unwrap();
///
/// println!("{}", dot_string);
/// ```
pub fn generate_dot_string(graph: &RailwayGraph) -> Result<String, Box<dyn Error>> {
    let dot = Dot::with_config(&graph.graph, &[Config::EdgeNoLabel]);
    Ok(format!("{:?}", dot))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::importer::overpass_importer::{
        from_railway_elements, Coordinate, ElementType, RailwayElement,
    };

    use super::*;

    #[test]
    fn test_generate_dot_string() {
        let elements = vec![
            RailwayElement {
                id: 1,
                element_type: ElementType::Way,
                lat: None,
                lon: None,
                tags: None,
                nodes: Some(vec![2, 3]),
                geometry: Some(vec![
                    Coordinate {
                        lat: 50.0,
                        lon: 8.0,
                    },
                    Coordinate {
                        lat: 51.0,
                        lon: 9.0,
                    },
                ]),
            },
            RailwayElement {
                id: 2,
                element_type: ElementType::Node,
                lat: Some(50.0),
                lon: Some(8.0),
                tags: None,
                nodes: None,
                geometry: None,
            },
            RailwayElement {
                id: 3,
                element_type: ElementType::Node,
                lat: Some(51.0),
                lon: Some(9.0),
                tags: None,
                nodes: None,
                geometry: None,
            },
        ];

        let railway_graph = from_railway_elements(&elements);
        let dot_string = generate_dot_string(&railway_graph).unwrap();
        println!("{}", &dot_string);

        assert!(dot_string.contains("graph {"));
        assert!(dot_string.contains("0 [ label = \"RailwayNode { id: 2, lat: 50.0, lon: 8.0 }\" ]"));
        assert!(dot_string.contains("1 [ label = \"RailwayNode { id: 3, lat: 51.0, lon: 9.0 }\" ]"));
        assert!(dot_string.contains("0 -- 1 [ ]"));
        assert!(dot_string.contains('}'));
    }

    #[test]
    fn test_generate_svg_string() {
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
                lat: Some(50.1122),
                lon: Some(8.6833),
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
                        lat: 50.1122,
                        lon: 8.6833,
                    },
                ]),
            },
        ];

        let railway_graph = from_railway_elements(&elements);
        let svg_string = generate_svg_string(&railway_graph).unwrap();

        assert!(svg_string.contains("<svg"));
        assert!(svg_string.contains("</svg>"));
        assert!(svg_string.contains("<circle"));
        assert!(svg_string.contains("<path"));
    }
}
