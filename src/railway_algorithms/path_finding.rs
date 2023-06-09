use crate::{
    prelude::RailwayGraphExt,
    railway_model::RailwayGraph,
    types::{EdgeId, NodeId},
};
use petgraph::algo::dijkstra;
use transit_grid::algorithms::ShortestPath;

/// `PathFinding` trait provides pathfinding algorithms for railway networks.
pub trait PathFinding {
    /// Calculate the shortest path distance between two nodes.
    ///
    /// # Arguments
    /// * `source` - The ID of the source node.
    /// * `target` - The ID of the target node.
    ///
    /// # Returns
    /// Returns the distance of the shortest path between the source and target nodes if it exists.
    fn shortest_path_distance(&self, source: NodeId, target: NodeId) -> Option<f64>;

    /// Calculate the shortest path between two nodes as a list of node IDs.
    ///
    /// # Arguments
    /// * `start` - The ID of the start node.
    /// * `end` - The ID of the end node.
    ///
    /// # Returns
    /// Returns a `Vec<i64>` containing the IDs of the nodes in the shortest path if it exists.
    /// The returned vector includes the start and end node IDs.
    fn shortest_path_nodes(&self, start: NodeId, end: NodeId) -> Option<Vec<NodeId>>;

    /// Calculate the shortest path between two nodes as a list of edge IDs.
    ///
    /// # Arguments
    /// * `start` - The ID of the start node.
    /// * `end` - The ID of the end node.
    ///
    /// # Returns
    /// Returns a `Vec<i64>` containing the IDs of the edges in the shortest path if it exists.
    fn shortest_path_edges(&self, start: NodeId, end: NodeId) -> Option<Vec<EdgeId>>;
}

impl PathFinding for RailwayGraph {
    fn shortest_path_distance(&self, source: NodeId, target: NodeId) -> Option<f64> {
        let source_index = self.physical_graph.id_to_index(source);
        let target_index = self.physical_graph.id_to_index(target);

        if let (Some(&source_index), Some(&target_index)) = (source_index, target_index) {
            let shortest_path = dijkstra(
                &self.physical_graph.graph,
                source_index,
                Some(target_index),
                |e| e.weight().length,
            );

            shortest_path.get(&target_index).copied()
        } else {
            None
        }
    }

    fn shortest_path_nodes(&self, start: NodeId, end: NodeId) -> Option<Vec<NodeId>> {
        self.find_shortest_path(start, end)
    }

    fn shortest_path_edges(&self, start: NodeId, end: NodeId) -> Option<Vec<EdgeId>> {
        let node_path = self.shortest_path_nodes(start, end)?;
        if node_path.len() < 2 {
            return None;
        }

        node_path
            .windows(2)
            .filter_map(|pair| {
                let edge = self.railway_edge(pair[0], pair[1])?;
                Some(edge.id)
            })
            .collect::<Vec<EdgeId>>()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use transit_grid::prelude::TransitNetworkRepairer;

    use crate::{
        importer::overpass_importer::from_railway_elements,
        railway_algorithms::{tests::test_elements, PathFinding},
    };

    #[test]
    fn test_shortest_path_distance() {
        let railway_graph = from_railway_elements(&test_elements());

        assert_relative_eq!(
            railway_graph.shortest_path_distance(1, 2).unwrap(),
            1322.421,
            epsilon = 0.1
        );
        assert_relative_eq!(
            railway_graph.shortest_path_distance(1, 3).unwrap(),
            3134.2,
            epsilon = 0.1
        );
        assert_relative_eq!(
            railway_graph.shortest_path_distance(2, 3).unwrap(),
            1811.801
        );
        assert_eq!(railway_graph.shortest_path_distance(1, 4), None);
    }

    #[test]
    fn test_shortest_path_nodes() {
        let mut railway_graph = from_railway_elements(&test_elements());
        railway_graph.repair();
        railway_graph.repair();

        assert_eq!(railway_graph.shortest_path_nodes(1, 2), Some(vec![1, 2]));
        assert_eq!(railway_graph.shortest_path_nodes(1, 3), Some(vec![1, 2, 3]));
        assert_eq!(railway_graph.shortest_path_nodes(2, 3), Some(vec![2, 3]));
        assert_eq!(railway_graph.shortest_path_nodes(1, 4), None);
    }

    #[test]
    fn test_shortest_path_edges() {
        let mut railway_graph = from_railway_elements(&test_elements());
        railway_graph.repair();
        railway_graph.repair();

        assert_eq!(railway_graph.shortest_path_edges(1, 2), Some(vec![4]));
        assert_eq!(railway_graph.shortest_path_edges(1, 3), Some(vec![4, 5]));
        assert_eq!(railway_graph.shortest_path_edges(2, 3), Some(vec![5]));
        assert_eq!(railway_graph.shortest_path_edges(1, 4), None);
    }
}
