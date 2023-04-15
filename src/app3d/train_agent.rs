use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy_polyline::prelude::*;
use geo_types::coord;
use rand::seq::SliceRandom;

use super::{AppResource, Node};
use crate::{
    ai::{TrainAgentAI, TrainAgentState},
    prelude::{RailwayEdge, RailwayGraph},
    railway_algorithms::PathFinding,
    railway_objects::{Moveable, NextTarget, RailwayObject, Train},
    simulation::agents::RailMovableAction,
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
    pub id: i64,
    pub train: Train,
    pub current_edge: Option<RailwayEdge>,
    pub edge_progress: f64,
    pub remaining_distance: f64, // Distance in meters
    pub ai_agent: Option<TrainAgentAI>,
}

impl TrainAgent {
    pub fn new(id: i64, current_node_id: Option<i64>, target_node_id: Option<i64>) -> Self {
        Self {
            id,
            train: Train {
                id,
                position: current_node_id,
                geo_location: None,
                next_target: target_node_id,
                targets: VecDeque::new(),
                speed: 20.0,
                ..Default::default()
            },
            current_edge: None,
            edge_progress: 0.0,
            remaining_distance: 0.0,
            ai_agent: None,
        }
    }

    pub fn on_node(current_node_id: i64) -> Self {
        let id = TRAIN_AGENT_ID.fetch_add(1, Ordering::SeqCst);
        Self::new(id, Some(current_node_id), None)
    }

    pub fn train(&mut self, railway_graph: &RailwayGraph, iterations: usize) {
        let initial_state = TrainAgentState {
            delta_distance_mm: 0,
            current_speed_mm_s: 0,
            max_speed_percentage: 0,
        };
        let ai_agent = TrainAgentAI::new(railway_graph.clone(), initial_state);
        self.ai_agent = Some(ai_agent);
        if let Some(ai_agent) = &mut self.ai_agent {
            ai_agent.train(iterations);
        }
    }

    pub fn remaining_distance(&self, railway_graph: &RailwayGraph) -> Option<f64> {
        if let (Some(current_node_id), Some(target_node_id)) =
            (self.train.position(), self.train.next_target())
        {
            if current_node_id == target_node_id {
                Some(0.0)
            } else {
                let remaining_path_distance =
                    railway_graph.shortest_path_distance(current_node_id, target_node_id)?;
                Some(remaining_path_distance)
            }
        } else {
            None
        }
    }
}

pub fn create_train_agent_bundle(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> impl FnOnce(&mut ChildBuilder) {
    let main_body = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(20.0, 10.0, 4.0))),
        material: materials.add(Color::rgb(0.0, 0.6, 0.0).into()),
        ..Default::default()
    };

    let top_part = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(6.0, 4.0, 2.0))),
        material: materials.add(Color::rgb(0.0, 0.4, 0.0).into()),
        transform: Transform::from_xyz(10.0, 0.0, 1.0),
        ..Default::default()
    };

    let bottom_part = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(6.0, 4.0, 2.0))),
        material: materials.add(Color::rgb(0.0, 0.4, 0.0).into()),
        transform: Transform::from_xyz(-10.0, 0.0, 1.0),
        ..Default::default()
    };

    let left_wheel = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cylinder {
            radius: 2.0,
            height: 4.0,
            resolution: 20,
            segments: 1,
        })),
        material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
        transform: Transform::from_xyz(-8.0, -5.0, 1.0),
        ..Default::default()
    };

    let right_wheel = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cylinder {
            radius: 2.0,
            height: 4.0,
            resolution: 20,
            segments: 1,
        })),
        material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
        transform: Transform::from_xyz(8.0, -5.0, 1.0),
        ..Default::default()
    };

    move |builder: &mut ChildBuilder| {
        builder
            .spawn((main_body, PickableBundle::default()))
            .with_children(|parent| {
                parent.spawn(top_part);
                parent.spawn(bottom_part);
                parent.spawn(left_wheel);
                parent.spawn(right_wheel);
            });
    }
}

pub fn train_agent_system(
    mut train_agent_query: Query<(&mut TrainAgent, &mut Transform)>,
    app_resource: Res<AppResource>,
    projection: Res<super::Projection>,
    time: Res<Time>,
) {
    if let Some(ref railway_graph) = app_resource.graph {
        for (mut train_agent, mut transform) in train_agent_query.iter_mut() {
            let (current_node_id, target_node_id, current_speed) = (
                train_agent.train.position(),
                train_agent.train.next_target(),
                train_agent.train.speed(),
            );
            make_train_observation(
                &mut train_agent.ai_agent,
                current_node_id,
                target_node_id,
                current_speed,
                &time,
            );
            update_train_speed(&mut train_agent, &time);

            if let Some(current_node_id) = train_agent.train.position() {
                update_train_target(&mut train_agent, railway_graph);
                if let Some(target_node_id) = train_agent.train.next_target() {
                    let current_edge = train_agent.current_edge.clone();
                    if let Some(edge) = current_edge {
                        let edge_progress = train_agent.edge_progress
                            + train_agent.train.speed() * time.delta_seconds() as f64;
                        update_train_position(
                            &mut train_agent,
                            &mut transform,
                            &edge,
                            time.delta_seconds() as f64,
                            &projection,
                            &app_resource,
                        );
                        train_agent.edge_progress = edge_progress;
                    } else if let Some(edge) =
                        railway_graph.railway_edge(current_node_id, target_node_id)
                    {
                        train_agent.current_edge = Some(edge.clone());
                        train_agent.edge_progress = 0.0;
                    }
                }
            }
        }
    }
}

/// Updates the target node of a train agent, if necessary.
///
/// # Arguments
///
/// * `train_agent` - A reference to the train agent to update.
/// * `railway_graph` - A reference to the `RailwayGraph` resource containing information about the railway network.
///
fn update_train_target(train_agent: &mut TrainAgent, railway_graph: &RailwayGraph) {
    if let Some(target_node_id) = train_agent.train.next_target() {
        if train_agent.train.position().unwrap() == target_node_id {
            train_agent.train.set_next_target(None);
            train_agent.current_edge = None;
        } else {
            if let Some(path) = railway_graph
                .shortest_path_nodes(train_agent.train.position.unwrap(), target_node_id)
            {
                if !path.is_empty() {
                    train_agent.train.position = Some(path[1]);
                    if path.len() == 2 {
                        train_agent.train.set_next_target(None);
                    }
                }
            }
            // Set the current edge if it's not already set
            if train_agent.current_edge.is_none() {
                if let Some(edge) = railway_graph
                    .railway_edge(train_agent.train.position().unwrap(), target_node_id)
                {
                    train_agent.current_edge = Some(edge.clone());
                    train_agent.edge_progress = 0.0;
                }
            }
        }
    } else {
        let reachable_nodes = railway_graph.reachable_nodes(train_agent.train.position().unwrap());
        if !reachable_nodes.is_empty() {
            let mut rng = rand::thread_rng();
            train_agent
                .train
                .set_next_target(Some(*reachable_nodes.choose(&mut rng).unwrap()));
        }
    }
}

fn make_train_observation(
    ai_agent: &mut Option<TrainAgentAI>,
    current_node_id: Option<i64>,
    target_node_id: Option<i64>,
    current_speed: f64,
    time: &Time,
) {
    if let (Some(ai_agent), Some(current_node_id)) = (ai_agent, current_node_id) {
        let delta_distance = current_speed * time.delta_seconds_f64() * 1000.0;
        ai_agent.observe(
            current_node_id,
            target_node_id,
            Some((current_speed * 1000.0) as i32),
            Some(delta_distance as i32),
        );
    }
}

fn update_train_speed(train_agent: &mut TrainAgent, time: &Time) {
    if let Some(ai_agent) = &train_agent.ai_agent {
        let action = ai_agent.best_action(&ai_agent.agent_rl.state);
        match action {
            Some(RailMovableAction::Stop) => {
                //train_agent.speed *= 0.9;
            }
            Some(RailMovableAction::AccelerateForward { acceleration }) => {
                train_agent.train.set_speed(
                    train_agent.train.speed()
                        + acceleration as f64 * time.raw_delta_seconds_f64() / 1000.0,
                );
            }
            Some(RailMovableAction::AccelerateBackward { acceleration }) => {
                train_agent.train.set_speed(
                    train_agent.train.speed()
                        - acceleration as f64 * time.delta_seconds_f64() / 1000.0,
                );
            }
            _ => (),
        }
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
fn update_train_position(
    train_agent: &mut TrainAgent,
    transform: &mut Transform,
    edge: &RailwayEdge,
    time_delta: f64,
    projection: &super::Projection,
    app_resource: &Res<AppResource>,
) {
    if let (Some(start_coord), Some(end_coord)) =
        (edge.path.coords().next(), edge.path.coords().last())
    {
        let edge_length = edge.length;

        let distance = train_agent.train.speed() * time_delta;
        let edge_progress = train_agent.edge_progress + distance;
        if edge_progress < edge_length {
            let progress_ratio = edge_progress / edge_length;
            train_agent.edge_progress = progress_ratio;

            let new_coord = coord! {
                x: start_coord.x + (end_coord.x - start_coord.x) * progress_ratio,
                y: start_coord.y + (end_coord.y - start_coord.y) * progress_ratio,
            };

            if let Some(view_coord) = projection.project(new_coord) {
                transform.translation.x = view_coord.x;
                transform.translation.y = view_coord.y;
            }
        } else {
            // The train has reached the end of the edge
            if let Some(target_node_id) = train_agent.train.next_target() {
                train_agent.train.position = Some(target_node_id);
                train_agent.train.set_next_target(None);
                train_agent.current_edge = None;
                train_agent.edge_progress = 0.0;
            }
        }
    }
    if let Some(graph) = &app_resource.graph {
        if let Some(distance) = train_agent.remaining_distance(graph) {
            train_agent.remaining_distance = distance;
        }
    }
}

pub fn train_agent_line_system(
    mut commands: Commands,
    train_agent_query: Query<(&TrainAgent, &Transform)>,
    node_query: Query<(&Node, &Transform), Without<TrainAgent>>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    q_line: Query<Entity, With<TrainAgentLine>>,
) {
    for entity in q_line.iter() {
        commands.entity(entity).despawn();
    }
    for (train_agent, train_agent_transform) in train_agent_query.iter() {
        if let (Some(current_node_id), Some(target_node_id)) = (
            train_agent.train.position(),
            train_agent.train.next_target(),
        ) {
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
                commands
                    .spawn(PolylineBundle {
                        polyline: polylines.add(Polyline {
                            vertices: vec![
                                train_agent_transform.translation,
                                //current_node_transform.translation,
                                target_node_transform.translation,
                            ],
                        }),
                        material: polyline_materials.add(PolylineMaterial {
                            width: 2.0,
                            color: Color::RED,
                            perspective: false,
                            ..default()
                        }),
                        ..default()
                    })
                    .insert(TrainAgentLine);
            }
        }
    }
}
