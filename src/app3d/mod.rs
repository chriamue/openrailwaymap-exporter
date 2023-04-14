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
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::PickingEvent;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle};
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

/// Keeps track of the currently selected start and end nodes.
#[derive(Default, Resource)]
pub struct SelectedTrain {
    train_agent_id: Option<i32>,
}

/// Defines the different interaction modes for the application.
#[derive(Debug, PartialEq, Clone, Default)]
pub enum InteractionMode {
    /// Default mode: selecting nodes to display information or find the shortest path.
    #[default]
    SelectMode,
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
        .add_plugins(DefaultPickingPlugins)
        .insert_resource(app_resource)
        .insert_resource(projection)
        .insert_resource(SelectedNode::default())
        .insert_resource(SelectedTrain::default())
        .insert_resource(InteractionModeResource::default())
        .add_startup_system(setup)
        .add_startup_system(display_graph)
        .add_system(ui::ui_system)
        .add_system(update_look_at_position_system)
        .add_system(select_node_system)
        .add_system(select_train_system)
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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Railwaymap".into(),
                resolution: (1000., 1000.).into(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugins(DefaultPickingPlugins)
        .insert_resource(AppResource::default())
        .insert_resource(projection)
        .insert_resource(SelectedNode::default())
        .insert_resource(SelectedTrain::default())
        .insert_resource(InteractionModeResource::default())
        .add_startup_system(setup)
        .add_system(ui::ui_system)
        .add_system(update_look_at_position_system)
        .add_system(select_node_system)
        .add_system(select_train_system)
        .add_system(show_path)
        .add_system(train_agent::train_agent_system)
        .run()
}

fn setup(mut commands: Commands) {
    //commands
    //    .spawn(Camera2dBundle::default())
    //    .insert(PanCam::default());
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PickingCameraBundle::default(),
    ));
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.7,
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 500.0, 0.0),
        point_light: PointLight {
            intensity: 10000.0,
            range: 1000.0,
            ..Default::default()
        },
        ..Default::default()
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

fn show_path(
    app_resource: Res<AppResource>,
    mut edge_query: Query<(&Edge, &mut Handle<StandardMaterial>)>,
    selected_node: Res<SelectedNode>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(graph) = &app_resource.graph {
        if let (Some(start_node_id), Some(end_node_id)) =
            (selected_node.start_node_id, selected_node.end_node_id)
        {
            // Use graph.shortest_path_edges to get the Vec of edge IDs
            if let Some(path_edge_ids) = graph.shortest_path_edges(start_node_id, end_node_id) {
                // Iterate through the edges and set their color
                for (edge, mut material_handle) in edge_query.iter_mut() {
                    let edge_data = edge;
                    let is_path_edge = path_edge_ids
                        .iter()
                        .any(|railway_edge| *railway_edge == edge_data.id);
                    if let Some(material) = materials.get_mut(&material_handle) {
                        material.base_color = if is_path_edge {
                            Color::RED
                        } else {
                            Color::BLUE
                        };
                    } else if is_path_edge {
                        let new_material = StandardMaterial {
                            base_color: Color::RED,
                            ..Default::default()
                        };
                        let new_material_handle = materials.add(new_material);
                        *material_handle = new_material_handle;
                    }
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
    mut materials: ResMut<Assets<StandardMaterial>>,
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
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box {
                            min_x: 0.0,
                            max_x: distance,
                            min_y: -0.5,
                            max_y: 0.5,
                            min_z: -0.5,
                            max_z: 0.5,
                        })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::BLUE,
                            ..Default::default()
                        }),
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
                    .spawn((
                        PbrBundle {
                            mesh: meshes.add(
                                Mesh::try_from(shape::Icosphere {
                                    radius: 5.0,
                                    subdivisions: 2,
                                })
                                .unwrap(),
                            ),
                            material: materials.add(StandardMaterial {
                                base_color: Color::RED,
                                ..Default::default()
                            }),
                            transform: Transform::from_translation(position),
                            ..Default::default()
                        },
                        PickableBundle::default(),
                    ))
                    .insert(Node { id: node_data.id });
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn select_node_system(
    mut events: EventReader<PickingEvent>,
    app_resource: Res<AppResource>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut selected_node: ResMut<SelectedNode>,
    q_node: Query<(Entity, &Node, &Transform), Without<Camera>>,
    interaction_mode: Res<InteractionModeResource>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut selection = None;
    for event in events.iter() {
        match event {
            PickingEvent::Selection(_e) => (),
            PickingEvent::Hover(_e) => (),
            PickingEvent::Clicked(e) => {
                for (entity, node, transform) in q_node.iter() {
                    if e == &entity {
                        selection = Some((entity, node.id, *transform));
                    }
                }
            }
        }
    }

    if let Some((entity, id, transform)) = selection {
        // Check the current interaction mode
        match interaction_mode.mode {
            InteractionMode::SelectMode => {
                println!("Selected node: {:?}", entity);
                selected_node.end_node_id = selected_node.start_node_id;
                selected_node.start_node_id = Some(id);

                for mut camera_transform in camera_query.iter_mut() {
                    camera_transform.translation.x = transform.translation.x;
                    camera_transform.translation.y = transform.translation.y;
                }
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
                        //PickableBundle::default()
                    ))
                    .insert(PickableBundle::default())
                    .with_children(train_agent::create_train_agent_bundle(meshes, materials));
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn select_train_system(
    mut events: EventReader<PickingEvent>,
    q_train: Query<(Entity, &TrainAgent, &Children)>,
    mut selected_train: ResMut<SelectedTrain>,
) {
    let mut selection = None;
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
            PickingEvent::Clicked(e) => {
                println!("Clicked: {:?}", e);
                for (entity, train, children) in q_train.iter() {
                    if e == &entity {
                        selection = Some(train.id);
                    } else {
                        for entity in children.iter() {
                            println!("{:?}, {:?}", entity, train.id);
                            if e == entity {
                                selection = Some(train.id);
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(id) = selection {
        selected_train.train_agent_id = Some(id);
    }
}
