#![cfg_attr(
    target_arch = "wasm32",
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]

use super::nodes::SelectedNode;
use super::train_agent::TrainAgent;
use super::{display_graph, SelectedTrain};
use super::{AppResource, Edge, Node, Projection};
use super::{InteractionMode, InteractionModeResource};
use crate::prelude::OverpassApiClient;
use crate::prelude::OverpassImporter;
use crate::prelude::RailwayApiClient;
use crate::prelude::RailwayGraph;
use crate::prelude::RailwayGraphImporter;
use crate::railway_algorithms::PathFinding;
use crate::railway_objects::{NextTarget, RailwayObject};
use bevy::prelude::Commands;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[derive(Default, Resource)]
pub struct UiUpdateTimer {
    pub time: f64,
    pub last_displayed_remaining: f64,
}

pub fn add_ui_systems_to_app(app: &mut App) {
    #[cfg(not(target_arch = "wasm32"))]
    app.add_system(select_graph_ui_system);
    app.add_system(selection_ui_system);
    app.insert_resource(UiUpdateTimer::default());
}

#[allow(clippy::too_many_arguments)]
pub fn selection_ui_system(
    mut contexts: EguiContexts,
    app_resource: ResMut<AppResource>,
    selected_node: Res<SelectedNode>,
    selected_train: Res<SelectedTrain>,
    q_train: Query<&TrainAgent>,
    mut interaction_mode: ResMut<InteractionModeResource>,
    time: Res<Time>,
    mut ui_update_timer: ResMut<UiUpdateTimer>,
) {
    if ui_update_timer.time >= 1.0 {
        ui_update_timer.time = 0.0;
    } else {
        ui_update_timer.time += time.delta_seconds_f64();
    }
    egui::Window::new("").show(contexts.ctx_mut(), |ui| {
        if let Some(node_id) = selected_node.start_node_id {
            if let Some(graph) = &app_resource.graph {
                display_selected_node_info(ui, graph, node_id);
            }
        } else {
            ui.label("No node selected");
        }
        ui.add_space(15.0); // Add space
        if let (Some(start_node_id), Some(end_node_id)) =
            (selected_node.start_node_id, selected_node.end_node_id)
        {
            if let Some(graph) = &app_resource.graph {
                display_path_info(ui, graph, start_node_id, end_node_id);
            }
        }
        ui.add_space(15.0); // Add space
        if let Some(train_agent_id) = selected_train.train_agent_id {
            for train_agent in q_train.iter() {
                if train_agent_id == train_agent.id {
                    display_selected_train_agent_info(ui, train_agent, &mut ui_update_timer);
                }
            }
        }

        // Add radio buttons for click action modes
        ui.add_space(15.0); // Add space
        ui.label("Click action mode:");
        ui.radio_value(
            &mut interaction_mode.mode,
            InteractionMode::SelectMode,
            "Select Mode",
        );
        ui.radio_value(
            &mut interaction_mode.mode,
            InteractionMode::PlaceTrain,
            "Place Train",
        );
    });
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::too_many_arguments)]
pub fn select_graph_ui_system(
    mut contexts: EguiContexts,
    commands: Commands,
    mut app_resource: ResMut<AppResource>,
    edge_query: Query<Entity, With<Edge>>,
    node_query: Query<Entity, With<Node>>,
    mut projection: ResMut<Projection>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    egui::Window::new("Railway Area").show(contexts.ctx_mut(), |ui| {
        ui.label("Enter an Area:");
        ui.text_edit_singleline(&mut app_resource.area_name);
        ui.add_space(15.0);

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
                ui.set_enabled(false);
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

pub fn display_selected_train_agent_info(
    ui: &mut egui::Ui,
    train_agent: &TrainAgent,
    ui_update_timer: &mut UiUpdateTimer,
) {
    ui.label(format!("ID: {}", train_agent.id));
    ui.label(format!("Current: {:?}", train_agent.train.position()));
    ui.label(format!("Target: {:?}", train_agent.train.next_target()));
    ui.label(format!("Speed: {} km/h", train_agent.speed * 3.6));
    if ui_update_timer.time == 0.0 {
        ui_update_timer.last_displayed_remaining = train_agent.remaining_distance / 1000.0_f64;
    }

    ui.label(format!(
        "Remaining: {:.3} km",
        ui_update_timer.last_displayed_remaining
    ));
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
