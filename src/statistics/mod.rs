//! A module providing utility functions for calculating path lengths, remaining lengths, and the total length of a railway network.

use uom::si::{f64::Length, length::meter};

use crate::{prelude::RailwayGraph, types::EdgeId};

/// Calculate the total length of the given sequence of edges in the railway network.
///
/// # Arguments
///
/// * `graph` - A reference to the `RailwayGraph` instance.
/// * `edges` - A `Vec<EdgeId>` representing the sequence of edges.
///
/// # Returns
///
/// An `Option<Length>` representing the total length of the given sequence of edges in meters, if all edge IDs are valid.
pub fn path_length(graph: &RailwayGraph, edges: &Vec<EdgeId>) -> Option<Length> {
    let mut total_length = 0.0;

    for edge_id in edges {
        if let Some(edge) = graph.get_edge_by_id(*edge_id) {
            total_length += edge.length
        }
    }
    Some(Length::new::<meter>(total_length))
}

#[cfg(test)]
mod tests {
    use transit_grid::prelude::TransitNetworkRepairer;

    use super::*;
    use crate::{railway_algorithms::PathFinding, tests::test_graph_1};

    #[test]
    fn test_path_length() {
        let mut graph = test_graph_1();
        graph.repair();
        graph.repair();
        let edges = graph.shortest_path_edges(1, 9).unwrap();
        let length = path_length(&graph, &edges).unwrap();
        assert_eq!(length.get::<meter>(), 2765.236);
    }
}
