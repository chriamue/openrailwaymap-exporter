//! The `app3d` module provides functionality to visualize railway graphs in 3D.
//!
//! It includes functions to initialize a Bevy application with a given `RailwayGraph` or a default one,
//! and provides systems for displaying the graph, interacting with the user interface, and updating the camera.
//!
#![cfg_attr(target_arch = "wasm32", allow(dead_code, unused_imports))]

use crate::app3d::train_agent::TrainAgent;
use crate::prelude::RailwayGraph;
use crate::railway_algorithms::PathFinding;
use bevy::input::Input;
use bevy::prelude::shape::Circle;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_egui::EguiPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
use geo_types::coord;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::NodeRef;

mod train_agent;
mod ui;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod projection;
pub use projection::Projection;

/// Represents an edge in the railway graph.
#[derive(Component)]
pub struct Edge {
    id: i64,
}

/// Represents a node in the railway graph.
#[derive(Component)]
pub struct Node {
    id: i64,
}

/// Holds application state, including the area name, railway graph, and camera look-at position.
#[derive(Default, Resource)]
pub struct AppResource {
    area_name: String,
    graph: Option<RailwayGraph>,
    look_at_position: Option<Vec3>,
}

/// Keeps track of the currently selected start and end nodes.
#[derive(Default, Resource)]
pub struct SelectedNode {
    start_node_id: Option<i64>,
    end_node_id: Option<i64>,
}

/// Defines the different interaction modes for the application.
#[derive(Debug, PartialEq, Clone, Default)]
pub enum InteractionMode {
    /// Default mode: selecting nodes to display information or find the shortest path.
    #[default]
    SelectNode,
    /// Mode for placing trains on the railway network.
    PlaceTrain,
}

/// Stores the current interaction mode.
#[derive(Default, Resource)]
pub struct InteractionModeResource {
    mode: InteractionMode,
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
    let mut projection = Projection::new(5000.0, 5000.0);
    let (min_coord, max_coord) = graph.bounding_box();
    projection.set_bounding_box(min_coord, max_coord);

    let app_resource = AppResource {
        area_name: "".to_string(),
        graph: Some(graph),
        look_at_position: None,
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(PanCamPlugin::default())
        .insert_resource(app_resource)
        .insert_resource(projection)
        .insert_resource(SelectedNode::default())
        .insert_resource(InteractionModeResource::default())
        .add_startup_system(setup)
        .add_startup_system(display_graph)
        .add_system(ui::ui_system)
        .add_system(update_look_at_position_system)
        .add_system(select_node_system)
        .add_system(show_path)
        .add_system(train_agent::train_agent_system)
        .run()
}

/// Initializes the Bevy application with a default `RailwayGraph`.
///
/// This function sets up the Bevy application with the required plugins,
/// resources, and systems. It inserts a default `AppResource` and starts the
/// Bevy application. The user can load a `RailwayGraph` through the UI.
///
#[cfg(not(target_arch = "wasm32"))]
pub fn init() {
    let projection = Projection::new(1000.0, 1000.0);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(PanCamPlugin::default())
        .insert_resource(AppResource::default())
        .insert_resource(projection)
        .insert_resource(SelectedNode::default())
        .insert_resource(InteractionModeResource::default())
        .add_startup_system(setup)
        .add_system(ui::ui_system)
        .add_system(update_look_at_position_system)
        .add_system(select_node_system)
        .add_system(show_path)
        .add_system(train_agent::train_agent_system)
        .run()
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
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

fn show_path(
    app_resource: Res<AppResource>,
    mut edge_query: Query<(&Edge, &mut Sprite)>,
    selected_node: Res<SelectedNode>,
) {
    if let Some(graph) = &app_resource.graph {
        if let (Some(start_node_id), Some(end_node_id)) =
            (selected_node.start_node_id, selected_node.end_node_id)
        {
            // Use graph.shortest_path_edges to get the Vec of edge IDs
            if let Some(path_edge_ids) = graph.shortest_path_edges(start_node_id, end_node_id) {
                // Iterate through the edges and set their color
                for (edge, mut sprite) in edge_query.iter_mut() {
                    let edge_data = edge;
                    sprite.color = if path_edge_ids
                        .iter()
                        .any(|railway_edge| *railway_edge == edge_data.id)
                    {
                        Color::RED
                    } else {
                        Color::BLUE
                    };
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
                            let sprite_size = Vec2::new(distance, 1.0);
                            Sprite {
                                custom_size: Some(sprite_size),
                                color: Color::BLUE,
                                ..Default::default()
                            }
                        },

                        transform: Transform::from_translation(start)
                            .mul_transform(Transform::from_rotation(Quat::from_rotation_z(angle))),
                        ..Default::default()
                    })
                    .insert(Edge { id: edge_data.id });
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
                    .spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(Circle::new(5.).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::RED)),
                        transform: Transform::from_translation(position),
                        ..default()
                    })
                    .insert(Node { id: node_data.id });
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn select_node_system(
    app_resource: Res<AppResource>,
    mut selected_node: ResMut<SelectedNode>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut windows: Query<&mut Window>,
    q_node: Query<(Entity, &Node, &Transform)>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    interaction_mode: Res<InteractionModeResource>,
    mut commands: Commands,
) {
    if mouse_button_inputs.just_pressed(MouseButton::Left) {
        let window = windows.single_mut();
        let (camera, camera_transform) = camera_q.single();

        let mut closest = None;
        let mut min_distance = f32::MAX;

        for (entity, node, transform) in q_node.iter() {
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin)
            {
                let world_position_2d = Vec2::new(world_position.x, world_position.y);
                let transform_2d = Vec2::new(transform.translation.x, transform.translation.y);
                let distance = world_position_2d.distance(transform_2d);

                if distance < 15.0 && distance < min_distance {
                    min_distance = distance;
                    closest = Some((entity, node.id, *transform));
                }
            }
        }

        if let Some((entity, id, transform)) = closest {
            // Check the current interaction mode
            match interaction_mode.mode {
                InteractionMode::SelectNode => {
                    println!("Selected node: {:?}", entity);
                    selected_node.end_node_id = selected_node.start_node_id;
                    selected_node.start_node_id = Some(id);
                }
                InteractionMode::PlaceTrain => {
                    println!("Placing train on node: {:?}", id);
                    let mut train_agent = TrainAgent::on_node(id);
                    if let Some(graph) = &app_resource.graph {
                        train_agent.train(graph, 100000);
                    }
                    commands
                        .spawn((
                            Transform::from_xyz(
                                transform.translation.x,
                                transform.translation.y,
                                transform.translation.z + 1.0,
                            ),
                            GlobalTransform::default(),
                            ComputedVisibility::default(),
                            Visibility::Inherited,
                            train_agent,
                        ))
                        .with_children(train_agent::create_train_agent_sprite_bundle());
                }
            }
        } else {
            selected_node.start_node_id = None;
        }
    }
}
