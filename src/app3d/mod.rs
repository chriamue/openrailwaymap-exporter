//! The `app3d` module provides functionality to visualize railway graphs in 3D.
//!
//! It includes functions to initialize a Bevy application with a given `RailwayGraph` or a default one,
//! and provides systems for displaying the graph, interacting with the user interface, and updating the camera.
//!
#![cfg_attr(target_arch = "wasm32", allow(dead_code, unused_imports))]

use std::sync::{Arc, RwLock};

use crate::app3d::train_agent::TrainAgent;
use crate::prelude::{RailwayGraph, RailwayGraphExt};
use crate::simulation::Simulation;
use bevy::input::Input;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::PickingEvent;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle};
use bevy_obj::ObjPlugin;
use bevy_polyline::prelude::{Polyline, PolylineBundle, PolylineMaterial};
use bevy_polyline::PolylinePlugin;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::NodeRef;

mod camera;
mod edges;
mod nodes;
mod train_agent;
mod ui;

use edges::Edge;
use nodes::Node;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod console;
mod projection;

pub use projection::Projection;

use self::train_agent::SelectedTrain;

/// Holds application state, including the area name, railway graph, and camera look-at position.
#[derive(Default, Resource)]
pub struct AppResource {
    area_name: String,
    graph: Option<RailwayGraph>,
    simulation: Option<Arc<RwLock<Simulation>>>,
    look_at_position: Option<Vec3>,
}

/// Stores configuration of current debug
#[derive(Default, Resource)]
pub struct DebugResource {
    show_train_target: bool,
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

/// Configures a Bevy application with the necessary plugins, resources, and systems.
///
/// This function configures a Bevy application with the required plugins,
/// resources, and systems based on the given `AppResource`. The caller is
/// responsible for inserting any additional resources and running the application.
///
/// # Arguments
///
/// * `app` - A mutable reference to a `bevy::prelude::App` instance to configure.
/// * `app_resource` - An `AppResource` to configure the application.
///
pub fn setup_app(app: &mut App, app_resource: AppResource) {
    app.add_plugins(camera::CameraPlugins)
        .add_plugin(EguiPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(PolylinePlugin)
        .add_plugin(ObjPlugin)
        .insert_resource(app_resource)
        .insert_resource(nodes::SelectedNode::default())
        .insert_resource(SelectedTrain::default())
        .insert_resource(InteractionModeResource::default())
        .insert_resource(DebugResource::default())
        .add_startup_system(setup)
        .add_startup_system(camera::setup_camera)
        .add_system(update_look_at_position_system)
        .add_system(nodes::select_node_system)
        .add_system(select_train_system)
        .add_system(edges::show_edges_on_path)
        .add_system(train_agent::update_train_position_system)
        .add_system(train_agent::update_train_agent_line_system)
        .add_system(update_simulation_system);
    ui::add_ui_systems_to_app(app);
    console::add_console_to_app(app);
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
        graph: Some(graph.clone()),
        look_at_position: None,
        simulation: Some(Arc::new(RwLock::new(Simulation::new(graph)))),
    };
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Railwaymap".into(),
            resolution: (1000., 1000.).into(),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: true,
            canvas: Some("#bevy".to_string()),
            ..default()
        }),
        ..default()
    }));
    setup_app(&mut app, app_resource);
    app.insert_resource(projection)
        .add_startup_system(display_graph)
        .run()
}

/// Initializes the Bevy application with a default `RailwayGraph`.
///
/// This function sets up the Bevy application with the required plugins,
/// resources, and systems. It inserts a default `AppResource` and starts the
/// Bevy application. The user can load a `RailwayGraph` through the UI.
///
/// # Note
///
/// This function is not available when compiling for the `wasm32` target.
#[cfg(not(target_arch = "wasm32"))]
pub fn init() {
    let projection = Projection::new(1000.0, 1000.0);
    let app_resource = AppResource::default();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Railwaymap".into(),
            resolution: (1000., 1000.).into(),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }));
    setup_app(&mut app, app_resource);
    app.insert_resource(projection).run()
}

fn setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.7,
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 500.0, 100.0),
        point_light: PointLight {
            intensity: 1000.0,
            range: 1000.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn update_simulation_system(app_resource: Res<AppResource>, time: Res<Time>) {
    if let Some(simulation) = &app_resource.simulation {
        simulation.write().unwrap().update(time.delta())
    }
}

fn update_look_at_position_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut app_resource: ResMut<AppResource>,
    projection: Res<Projection>,
) {
    if keyboard_input.just_pressed(KeyCode::N) {
        if let Some(graph) = &app_resource.graph {
            let mut nodes = graph.physical_graph.graph.node_references();
            let current_position = app_resource.look_at_position.unwrap_or(Vec3::ZERO);

            for transform in camera_query.iter() {
                println!("{:?}", transform);
            }

            let next_node = nodes
                .find(|(_, node_data)| {
                    let position = projection.project(node_data.location).unwrap_or_default();
                    position != current_position
                })
                .map(|(_, node_data)| projection.project(node_data.location).unwrap_or_default());

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
    edge_query: Query<Entity, With<edges::Edge>>,
    node_query: Query<Entity, With<nodes::Node>>,
    projection: Res<Projection>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
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
        for edge in graph.physical_graph.graph.edge_references() {
            let edge_data = edge.weight();
            let path = &edge_data.path.0;

            for coords in path.windows(2) {
                let start = projection.project(coords[0]).unwrap();
                let end = projection.project(coords[1]).unwrap();

                commands
                    .spawn(PolylineBundle {
                        polyline: polylines.add(Polyline {
                            vertices: vec![start, end],
                        }),
                        material: polyline_materials.add(PolylineMaterial {
                            width: 2.0,
                            color: Color::BLUE,
                            perspective: false,
                            depth_bias: -0.0002,
                        }),
                        ..Default::default()
                    })
                    .insert(Edge { id: edge_data.id });
            }
        }

        // Display nodes
        for node in graph.physical_graph.graph.node_references() {
            let node_data = node.weight();
            let position = projection.project(node_data.location);

            if let Some(position) = position {
                commands
                    .spawn((
                        PbrBundle {
                            mesh: meshes.add(
                                Mesh::try_from(shape::Icosphere {
                                    radius: 1.0,
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
                    .insert(nodes::Node { id: node_data.id });
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
            PickingEvent::Selection(_e) => (),
            PickingEvent::Hover(_e) => (),
            PickingEvent::Clicked(e) => {
                for (entity, train, children) in q_train.iter() {
                    if e == &entity {
                        selection = Some(train.id);
                    } else {
                        for entity in children.iter() {
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
