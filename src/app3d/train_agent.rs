use bevy::prelude::*;
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_mod_picking::prelude::{On, Pointer};
use bevy_mod_picking::{
    prelude::{Click, RaycastPickTarget},
    PickableBundle,
};
use uom::si::velocity::{kilometer_per_hour, meter_per_second, Velocity};

use super::{AppResource, Node};
use crate::app3d::DebugResource;
use crate::prelude::RailwayGraphExt;
use crate::railway_algorithms::RailwayGraphAlgos;
use crate::types::NodeId;
use crate::{
    prelude::{RailwayEdge, RailwayGraph},
    railway_objects::{GeoLocation, NextTarget, RailwayObject, Train},
    simulation::{agents::ForwardUntilTargetAgent, Simulation, SimulationObject},
    types::RailwayObjectId,
};
use std::{
    collections::VecDeque,
    sync::atomic::{AtomicI64, Ordering},
};

static TRAIN_AGENT_ID: AtomicI64 = AtomicI64::new(0);

/// Keeps track of the currently selected start and end nodes.
#[derive(Default, Resource)]
pub struct SelectedTrain {
    pub train_agent_id: Option<i64>,
}

#[derive(Component)]
pub struct TrainAgentLine;

#[derive(Component, Debug)]
pub struct TrainAgent {
    pub id: RailwayObjectId,
    pub current_edge: Option<RailwayEdge>,
    pub edge_progress: f64,
    pub remaining_distance: f64,
}

pub fn create_new_train_id() -> RailwayObjectId {
    TRAIN_AGENT_ID.fetch_add(1, Ordering::SeqCst)
}

impl TrainAgent {
    pub fn new(id: RailwayObjectId) -> Self {
        Self {
            id,
            current_edge: None,
            edge_progress: 0.0,
            remaining_distance: 0.0,
        }
    }
}

#[derive(Debug, Component, Event)]
pub struct TrainSelectedEvent(Entity);

impl From<ListenerInput<Pointer<Click>>> for TrainSelectedEvent {
    fn from(click_event: ListenerInput<Pointer<Click>>) -> Self {
        Self(click_event.target)
    }
}

pub fn select_train_system(
    mut events: EventReader<TrainSelectedEvent>,
    q_train: Query<(Entity, &TrainAgent, &Children)>,
    mut selected_train: ResMut<SelectedTrain>,
) {
    let mut selection = None;
    for event in events.iter() {
        for (_entity, train, children) in q_train.iter() {
            if children.iter().any(|child| child == &event.0) {
                selection = Some(train.id);
            }
        }
    }

    if let Some(id) = selection {
        selected_train.train_agent_id = Some(id);
    }
}

pub fn create_train(
    id: RailwayObjectId,
    position: Option<NodeId>,
    target: Option<NodeId>,
    simulation: &mut Simulation,
) -> RailwayObjectId {
    let geo_location = {
        let node = simulation
            .get_observable_environment()
            .get_graph()
            .get_node_by_id(position.unwrap());
        Some(node.unwrap().location)
    };

    let agent = ForwardUntilTargetAgent::new(id);
    let train = Train {
        id,
        position,
        geo_location,
        next_target: target,
        targets: VecDeque::new(),
        max_speed: Velocity::new::<kilometer_per_hour>(80.0),
        speed: Velocity::new::<meter_per_second>(20.0),
        ..Default::default()
    };
    let object: Box<dyn SimulationObject> = Box::new(train);

    simulation.add_object(object, Some(Box::new(agent)));
    id
}

pub fn create_train_agent_bundle(
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) -> impl FnOnce(&mut ChildBuilder) {
    let rotation_quat = Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2);

    let main_body = PbrBundle {
        mesh: asset_server.load("train.obj"),
        material: materials.add(Color::rgb(0.0, 0.6, 0.0).into()),
        transform: Transform {
            rotation: rotation_quat,
            scale: Vec3::ONE * 0.001,
            ..Default::default()
        },
        ..Default::default()
    };

    move |builder: &mut ChildBuilder| {
        builder.spawn((
            main_body,
            PickableBundle::default(),
            RaycastPickTarget::default(),
            On::<Pointer<Click>>::send_event::<TrainSelectedEvent>(),
        ));
    }
}

/// Updates the position of a train agent along its current railway edge.
///
/// # Arguments
///
/// * `train_agent` - A reference to the train agent to update.
/// * `transform` - A mutable reference to the `Transform` component of the train agent entity.
/// * `edge` - The current railway edge of the train agent.
/// * `edge_progress` - The current progress of the train agent along its current railway edge, in meters.
/// * `projection` - A reference to the `Projection` resource used to convert geographical coordinates to view coordinates.
///
pub fn update_train_position_system(
    mut train_agent_query: Query<(&TrainAgent, &mut Transform)>,
    app_resource: Res<AppResource>,
    projection: Res<super::Projection>,
) {
    for (train_agent, mut transform) in train_agent_query.iter_mut() {
        if let Some(train) = clone_train_from_app(train_agent, &app_resource) {
            if let Some(location) = train.geo_location() {
                if let Some(view_coord) = &projection.project(location) {
                    transform.translation.x = view_coord.x;
                    transform.translation.y = view_coord.y;
                    // Add next target information
                    if let Some(target_node_id) = train.next_target() {
                        if let Some(simulation) = &app_resource.simulation {
                            if let Ok(simulation) = simulation.read() {
                                let graph = simulation.get_observable_environment().get_graph();
                                if let Some(next_node_id) = graph.get_next_node(
                                    train.position().unwrap_or_default(),
                                    target_node_id,
                                ) {
                                    update_look_at(&projection, &mut transform, graph, next_node_id)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn update_look_at(
    projection: &Res<super::Projection>,
    transform: &mut Mut<Transform>,
    graph: &RailwayGraph,
    next_node_id: NodeId,
) {
    if let Some(next_node) = graph.get_node_by_id(next_node_id) {
        let target_location = next_node.location;
        if let Some(target_view_coord) = projection.project(target_location) {
            transform.look_at(
                Vec3::new(target_view_coord.x, target_view_coord.y, 1.0),
                Vec3::Z,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_train_agent_line_system(
    app_resource: Res<AppResource>,
    train_agent_query: Query<(&TrainAgent, &Transform)>,
    node_query: Query<(&Node, &Transform), Without<TrainAgent>>,
    mut gizmos: Gizmos,
    debug_resource: Res<DebugResource>,
) {
    if debug_resource.show_train_target {
        for (train_agent, train_agent_transform) in train_agent_query.iter() {
            if let Some(train) = clone_train_from_app(train_agent, &app_resource) {
                if let (Some(current_node_id), Some(target_node_id)) =
                    (train.position(), train.next_target())
                {
                    let current_node_transform = node_query
                        .iter()
                        .find(|(node, _)| node.id == current_node_id)
                        .map(|(_, transform)| transform);

                    let target_node_transform = node_query
                        .iter()
                        .find(|(node, _)| node.id == target_node_id)
                        .map(|(_, transform)| transform);

                    if let (Some(_current_node_transform), Some(target_node_transform)) =
                        (current_node_transform, target_node_transform)
                    {
                        gizmos.line(
                            train_agent_transform.translation,
                            target_node_transform.translation,
                            Color::RED,
                        );
                    }
                }
            }
        }
    }
}

pub fn clone_train_from_app(train_agent: &TrainAgent, app_resource: &AppResource) -> Option<Train> {
    if let Some(simulation) = &app_resource.simulation {
        if let Ok(simulation) = simulation.read() {
            if let Some(object) = simulation
                .get_observable_environment()
                .get_object(&train_agent.id)
            {
                if let Some(train) = object.as_any().downcast_ref::<Train>() {
                    return Some(train.clone());
                }
            }
        }
    }
    None
}
