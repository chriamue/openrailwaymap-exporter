use crate::{RailwayElement, RailwayGraph};
use petgraph::dot::{Config, Dot};
use serde_json;
use std::error::Error;

pub fn generate_dot_string(graph: &RailwayGraph) -> Result<String, Box<dyn Error>> {
    let dot = Dot::with_config(&graph.graph, &[Config::EdgeNoLabel]);
    Ok(format!("{:?}", dot))
}

pub fn generate_json_string(elements: &[RailwayElement]) -> Result<String, Box<dyn Error>> {
    let json_data = serde_json::to_string_pretty(elements)?;
    Ok(json_data)
}
