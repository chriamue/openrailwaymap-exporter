//! The `app3d` module provides functionality to visualize railway graphs in 3D.
//!
//! It includes functions to initialize a Bevy application with a given `RailwayGraph` or a default one,
//! and provides systems for displaying the graph, interacting with the user interface, and updating the camera.
//!
use crate::prelude::OverpassApiClient;
use crate::prelude::OverpassImporter;
use crate::prelude::RailwayApiClient;
use crate::prelude::RailwayGraph;
use crate::prelude::RailwayGraphImporter;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use geo_types::coord;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::NodeRef;
use tokio::runtime::Runtime;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod projection;
use projection::Projection;

// Components
#[derive(Component)]
struct Edge;
// Components
#[derive(Component)]
struct Node;

// Resources
#[derive(Default, Resource)]
struct AppResource {
    area_name: String,
    graph: Option<RailwayGraph>,
    look_at_position: Option<Vec3>,
}

/// Initializes the Bevy application with a given `RailwayGraph`.
///
/// This function sets up the Bevy application with the required plugins,
/// resources, and systems. It inserts the provided `RailwayGraph` into the
/// `AppResource` and starts the Bevy application.
///
/// # Arguments
///
/// * `graph` - A `RailwayGraph` to display in the application.
///
pub fn init_with_graph(graph: RailwayGraph) {
    let app_resource = AppResource {
        area_name: "".to_string(),
        graph: Some(graph),
        look_at_position: None,
    };
    let projection = Projection::new(1000.0, 1000.0);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(app_resource)
        .insert_resource(projection)
        .add_startup_system(setup)
        .add_system(update_look_at_position_system)
        .run()
}

/// Initializes the Bevy application with a default `RailwayGraph`.
///
/// This function sets up the Bevy application with the required plugins,
/// resources, and systems. It inserts a default `AppResource` and starts the
/// Bevy application. The user can load a `RailwayGraph` through the UI.
///
pub fn init() {
    let projection = Projection::new(1000.0, 1000.0);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(AppResource::default())
        .insert_resource(projection)
        .add_startup_system(setup)
        .add_system(ui_system)
        .add_system(update_look_at_position_system)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
}

fn ui_system(
    mut contexts: EguiContexts,
    commands: Commands,
    mut app_resource: ResMut<AppResource>,
    edge_query: Query<Entity, With<Edge>>,
    node_query: Query<Entity, With<Node>>,
    mut projection: ResMut<Projection>,
) {
    egui::Window::new("").show(&contexts.ctx_mut(), |ui| {
        ui.label("Enter an Area:");
        ui.text_edit_singleline(&mut app_resource.area_name);

        if ui.button("Load Railway Graph").clicked() {
            let area_name = app_resource.area_name.clone();
            // Process input and update Bevy resources or systems
            println!("Loading railway graph data: {}", area_name);

            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                let client = OverpassApiClient::new();

                let api_json_value = {
                    if area_name.contains(",") {
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
                );
            });
        }
    });
}

fn update_look_at_position_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut app_resource: ResMut<AppResource>,
    projection: Res<Projection>,
) {
    if keyboard_input.just_pressed(KeyCode::N) {
        if let Some(graph) = &app_resource.graph {
            let mut nodes = graph.graph.node_references();
            let current_position = app_resource.look_at_position.unwrap_or(Vec3::ZERO);

            for transform in camera_query.iter() {
                println!("{:?}", transform);
            }

            let next_node = nodes
                .find(|(_, node_data)| {
                    let position = projection
                        .project(coord! {
                            x: node_data.lon,
                            y: node_data.lat,
                        })
                        .unwrap_or_default();
                    position != current_position
                })
                .map(|(_, node_data)| {
                    projection
                        .project(coord! {
                            x: node_data.lon,
                            y: node_data.lat,
                        })
                        .unwrap_or_default()
                });

            if let Some(next_position) = next_node {
                app_resource.look_at_position = Some(next_position);
            }
            if let Some(look_at_position) = app_resource.look_at_position {
                for mut transform in camera_query.iter_mut() {
                    *transform = Transform::from_translation(transform.translation)
                        .looking_at(look_at_position, Vec3::Z);
                }
            }
        }
    }
}

fn display_graph(
    mut commands: Commands,
    app_resource: Res<AppResource>,
    edge_query: Query<Entity, With<Edge>>,
    node_query: Query<Entity, With<Node>>,
    projection: Res<Projection>,
) {
    if let Some(graph) = &app_resource.graph {
        // Clear previous edges and nodes
        for entity in edge_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in node_query.iter() {
            commands.entity(entity).despawn();
        }
        // Display edges
        for edge in graph.graph.edge_references() {
            let edge_data = edge.weight();
            let path = &edge_data.path.0;

            for coords in path.windows(2) {
                let start = projection.project(coords[0]).unwrap();
                let end = projection.project(coords[1]).unwrap();

                let diff = end - start;
                let distance = (diff.x * diff.x + diff.y * diff.y).sqrt();

                let angle = diff.y.atan2(diff.x);

                commands
                    .spawn(SpriteBundle {
                        sprite: {
                            let sprite_size = Vec2::new(distance as f32, 2.0);
                            let mut sprite = Sprite::default();
                            sprite.custom_size = Some(sprite_size);
                            sprite.color = Color::BLUE;
                            sprite
                        },

                        transform: Transform::from_translation(start)
                            .mul_transform(Transform::from_rotation(Quat::from_rotation_z(angle))),
                        ..Default::default()
                    })
                    .insert(Edge);
            }
        }

        // Display nodes
        for node in graph.graph.node_references() {
            let node_data = node.weight();
            let position = projection.project(coord! {
                x: node_data.lon,
                y: node_data.lat,
            });

            if let Some(position) = position {
                commands
                    .spawn(SpriteBundle {
                        sprite: {
                            let sprite_size = Vec2::new(5.0, 5.0);
                            let mut sprite = Sprite::default();
                            sprite.custom_size = Some(sprite_size);
                            sprite.color = Color::RED;
                            sprite
                        },
                        transform: Transform::from_translation(position),
                        ..Default::default()
                    })
                    .insert(Node);
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_app3d() {
    console_error_panic_hook::set_once();
    init();
}
