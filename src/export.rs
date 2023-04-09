use petgraph::dot::{Config, Dot};
use petgraph::visit::{IntoNodeReferences, NodeRef};
use std::error::Error;

use crate::prelude::RailwayGraph;

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
/// use openrailwaymap_exporter::prelude::overpass_api_client::RailwayElement;
/// use openrailwaymap_exporter::prelude::generate_dot_string;
/// use openrailwaymap_exporter::prelude::from_railway_elements;
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

/// Generates an SVG string representation of a given RailwayGraph.
///
/// The SVG string can be used to visualize the graph.
///
/// # Arguments
///
/// * `graph` - A reference to a RailwayGraph.
///
/// # Returns
///
/// A `Result` containing an SVG-formatted `String` on success, or a `Box<dyn Error>` on failure.
pub fn generate_svg_string(graph: &RailwayGraph) -> Result<String, Box<dyn Error>> {
    let (min_coord, max_coord) = graph.bounding_box();
    let width = 10000.0;
    let height = 10000.0;

    let x_scale = width / (max_coord.lon - min_coord.lon);
    let y_scale = height / (max_coord.lat - min_coord.lat);

    let mut svg_edges = String::new();
    let mut svg_nodes = String::new();

    for edge in graph.graph.edge_references() {
        let edge_data = edge.weight();

        let mut path_data = String::new();
        for (i, coord) in edge_data.path.0.iter().enumerate() {
            let x = (coord.x - min_coord.lon) * x_scale;
            let y = height - (coord.y - min_coord.lat) * y_scale;

            if i == 0 {
                path_data.push_str(&format!("M {} {}", x, y));
            } else {
                path_data.push_str(&format!(" L {} {}", x, y));
            }
        }

        svg_edges.push_str(&format!(
            r#"<path d="{}" stroke="black" stroke-width="2" fill="none" />"#,
            path_data
        ));
    }

    for node in graph.graph.node_references() {
        let node_data = node.weight();
        let x = (node_data.lon - min_coord.lon) * x_scale;
        let y = height - (node_data.lat - min_coord.lat) * y_scale;

        svg_nodes.push_str(&format!(
            r#"<circle cx="{}" cy="{}" r="5" fill="red" />"#,
            x, y
        ));
    }

    Ok(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}">
{}{}
</svg>"#,
        width, height, svg_edges, svg_nodes
    ))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::prelude::{
        from_railway_elements,
        overpass_api_client::{Coordinate, ElementType, RailwayElement},
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
        assert!(dot_string.contains("}"));
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
