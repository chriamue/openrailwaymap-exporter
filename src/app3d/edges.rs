use bevy::prelude::*;

use crate::{
    railway_algorithms::{PathFinding, RailwayGraphAlgos},
    types::EdgeId,
};

use super::{nodes::SelectedNode, AppResource, Projection};

/// Represents an edge in the railway graph.
#[derive(Component)]
pub struct Edge {
    pub id: EdgeId,
}

pub fn show_edges(
    app_resource: Res<AppResource>,
    selected_node: Res<SelectedNode>,
    projection: Res<Projection>,
    mut gizmos: Gizmos,
) {
    if let Some(graph) = &app_resource.graph {
        let mut highlighted_edges = Vec::new();
        if let (Some(start_node_id), Some(end_node_id)) =
            (selected_node.start_node_id, selected_node.end_node_id)
        {
            let path_edge_ids = if start_node_id == end_node_id {
                Some(graph.reachable_edges(start_node_id))
            } else {
                graph.shortest_path_edges(start_node_id, end_node_id)
            };
            // Use graph.shortest_path_edges to get the Vec of edge IDs
            if let Some(path_edge_ids) = path_edge_ids {
                highlighted_edges = path_edge_ids;
            }
        }
        for edge in graph.physical_graph.graph.edge_references() {
            let edge_data = edge.weight();
            let path = &edge_data.path.0;

            let points = path
                .iter()
                .map(|coords| projection.project(*coords).unwrap())
                .collect::<Vec<_>>();
            if highlighted_edges.contains(&edge_data.id) {
                gizmos.linestrip(points, bevy::color::palettes::css::RED);
            } else {
                gizmos.linestrip(points, bevy::color::palettes::css::BLUE);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_component_spawn_and_query() {
        let mut world = World::new();
        let entity = world.spawn(Edge { id: 42 }).id();

        let edge = world.get::<Edge>(entity).expect("Edge component missing");
        assert_eq!(edge.id, 42);
    }
}
