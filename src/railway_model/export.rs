use crate::railway_model::RailwayGraph;
use petgraph::dot::{Config, Dot};
use std::error::Error;

pub fn generate_dot_string(graph: &RailwayGraph) -> Result<String, Box<dyn Error>> {
    let dot = Dot::with_config(&graph.graph, &[Config::EdgeNoLabel]);
    Ok(format!("{:?}", dot))
}
