use crate::RailwayElement;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    node_a: i64,
    node_b: i64,
}

pub fn create_edges(elements: &[RailwayElement]) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::new();

    for element in elements {
        if let Some(nodes) = &element.nodes {
            for i in 0..(nodes.len() - 1) {
                let node_a = nodes[i];
                let node_b = nodes[i + 1];
                edges.push(Edge { node_a, node_b });
            }
        }
    }

    edges
}
