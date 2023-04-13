#![cfg_attr(
    target_arch = "wasm32",
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]

use super::display_graph;
use super::{AppResource, Edge, Node, Projection, SelectedNode};
use super::{InteractionMode, InteractionModeResource};
use crate::prelude::OverpassApiClient;
use crate::prelude::OverpassImporter;
use crate::prelude::RailwayApiClient;
use crate::prelude::RailwayGraph;
use crate::prelude::RailwayGraphImporter;
use crate::railway_algorithms::PathFinding;
use bevy::prelude::Commands;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[allow(clippy::too_many_arguments)]
pub fn ui_system(
    mut contexts: EguiContexts,
    commands: Commands,
    mut app_resource: ResMut<AppResource>,
    edge_query: Query<Entity, With<Edge>>,
    node_query: Query<Entity, With<Node>>,
    mut projection: ResMut<Projection>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    selected_node: Res<SelectedNode>,
    mut interaction_mode: ResMut<InteractionModeResource>,
) {
    egui::Window::new("").show(contexts.ctx_mut(), |ui| {
        if let Some(node_id) = selected_node.start_node_id {
            if let Some(graph) = &app_resource.graph {
                display_selected_node_info(ui, graph, node_id);
            }
        } else {
            ui.label("No node selected");
        }
        if let (Some(start_node_id), Some(end_node_id)) =
            (selected_node.start_node_id, selected_node.end_node_id)
        {
            if let Some(graph) = &app_resource.graph {
                display_path_info(ui, graph, start_node_id, end_node_id);
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            ui.label("Enter an Area:");
            ui.text_edit_singleline(&mut app_resource.area_name);

            if ui.button("Load Railway Graph").clicked() {
                let area_name = app_resource.area_name.clone();
                // Process input and update Bevy resources or systems
                println!("Loading railway graph data: {}", area_name);

                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async move {
                    let client = OverpassApiClient::new();

                    let api_json_value = {
                        if area_name.contains(',') {
                            client
                                .fetch_by_bbox(&area_name)
                                .await
                                .unwrap_or(client.fetch_by_area_name(&area_name).await.unwrap())
                        } else {
                            client.fetch_by_area_name(&area_name).await.unwrap()
                        }
                    };

                    let graph = OverpassImporter::import(&api_json_value).unwrap();
                    let (min_coord, max_coord) = graph.bounding_box();
                    projection.set_bounding_box(min_coord, max_coord);
                    app_resource.graph = Some(graph);
                    display_graph(
                        commands,
                        app_resource.into(),
                        edge_query,
                        node_query,
                        projection.into(),
                        meshes,
                        materials,
                    );
                });
            }
        }

        // Add radio buttons for click action modes
        ui.label("Click action mode:");
        ui.radio_value(
            &mut interaction_mode.mode,
            InteractionMode::SelectNode,
            "Select Node",
        );
        ui.radio_value(
            &mut interaction_mode.mode,
            InteractionMode::PlaceTrain,
            "Place Train",
        );
    });
}

pub fn display_selected_node_info(ui: &mut egui::Ui, graph: &RailwayGraph, node_id: i64) {
    if let Some(node_index) = graph.node_indices.get(&node_id) {
        let node = &graph.graph[*node_index];
        ui.label(format!("ID: {}", node.id));
        ui.label(format!("Latitude: {}", node.lat));
        ui.label(format!("Longitude: {}", node.lon));
    }
}

pub fn display_path_info(
    ui: &mut egui::Ui,
    graph: &RailwayGraph,
    start_node_id: i64,
    end_node_id: i64,
) {
    if let (Some(start_node_index), Some(end_node_index)) = (
        graph.node_indices.get(&start_node_id),
        graph.node_indices.get(&end_node_id),
    ) {
        let start_node = &graph.graph[*start_node_index];
        ui.label(format!("Start: {}", start_node.id));
        let end_node = &graph.graph[*end_node_index];
        ui.label(format!("End: {}", end_node.id));

        if graph
            .shortest_path_nodes(start_node_id, end_node_id)
            .is_some()
        {
            let distance = graph
                .shortest_path_distance(start_node_id, end_node_id)
                .map(|d| format!("{:.2} meters", d))
                .unwrap_or_else(|| "unknown".to_string());
            ui.label(format!("Distance: {}", distance));
        }
    }
}
