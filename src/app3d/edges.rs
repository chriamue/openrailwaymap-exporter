use bevy::prelude::*;
use bevy_polyline::prelude::PolylineMaterial;

use crate::{
    railway_algorithms::{PathFinding, RailwayGraphAlgos},
    types::EdgeId,
};

use super::{nodes::SelectedNode, AppResource};

/// Represents an edge in the railway graph.
#[derive(Component)]
pub struct Edge {
    pub id: EdgeId,
}

pub fn show_edges_on_path(
    app_resource: Res<AppResource>,
    mut edge_query: Query<(&Edge, &mut Handle<PolylineMaterial>)>,
    selected_node: Res<SelectedNode>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
) {
    if let Some(graph) = &app_resource.graph {
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
                // Iterate through the edges and set their color
                for (edge, mut material_handle) in edge_query.iter_mut() {
                    let edge_data = edge;
                    let is_path_edge = path_edge_ids
                        .iter()
                        .any(|railway_edge| *railway_edge == edge_data.id);
                    if let Some(material) = polyline_materials.get_mut(&material_handle) {
                        material.color = if is_path_edge {
                            Color::RED
                        } else {
                            Color::BLUE
                        };
                    }
                }
            }
        }
    }
}
