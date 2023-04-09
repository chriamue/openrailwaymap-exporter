mod railway_edge;
/// A module for working with railway graphs.
pub mod railway_graph;
mod railway_graph_builder;
mod railway_node;

pub use railway_edge::RailwayEdge;
pub use railway_graph::RailwayGraph;
pub use railway_graph_builder::{
    calculate_geometry_length, create_nodes, find_next_existing_node, from_railway_elements,
};
pub use railway_node::RailwayNode;
