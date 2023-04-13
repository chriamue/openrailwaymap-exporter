// train_agent.rs
use bevy::prelude::*;
use geo_types::coord;
use rand::seq::SliceRandom;

use super::{AppResource, Node};
use crate::{
    prelude::{RailwayEdge, RailwayGraph},
    railway_algorithms::PathFinding,
};
use std::sync::atomic::{AtomicI32, Ordering};

static TRAIN_AGENT_ID: AtomicI32 = AtomicI32::new(0);

#[derive(Component, Debug)]
pub struct TrainAgent {
    pub id: i32,
    pub current_node_id: Option<i64>,
    pub target_node_id: Option<i64>,
    pub current_edge: Option<RailwayEdge>,
    pub edge_progress: f64,
    pub speed: f64, // Speed in meters per second
}

impl TrainAgent {
    pub fn new(id: i32, current_node_id: Option<i64>, target_node_id: Option<i64>) -> Self {
        Self {
            id,
            current_node_id,
            target_node_id,
            current_edge: None,
            edge_progress: 0.0,
            speed: 20.0,
        }
    }
    pub fn on_node(current_node_id: i64) -> Self {
        let id = TRAIN_AGENT_ID.fetch_add(1, Ordering::SeqCst);
        Self::new(id, Some(current_node_id), None)
    }
}

pub fn create_train_agent_sprite_bundle() -> impl FnOnce(&mut ChildBuilder) {
    let main_body = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(20.0, 10.0)),
            color: Color::rgb(0.0, 0.6, 0.0),
            ..Default::default()
        },
        transform: Transform::from_scale(Vec3::new(2.0, 2.0, 1.0)),
        ..Default::default()
    };

    let top_part = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(6.0, 4.0)),
            color: Color::rgb(0.0, 0.4, 0.0),
            ..Default::default()
        },
        transform: Transform::from_xyz(10.0, 0.0, 0.0),
        ..Default::default()
    };

    let bottom_part = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(6.0, 4.0)),
            color: Color::rgb(0.0, 0.4, 0.0),
            ..Default::default()
        },
        transform: Transform::from_xyz(-10.0, 0.0, 0.0),
        ..Default::default()
    };

    let left_wheel = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(4.0, 4.0)),
            color: Color::rgb(0.2, 0.2, 0.2),
            ..Default::default()
        },
        transform: Transform::from_xyz(-8.0, -5.0, 0.0),
        ..Default::default()
    };

    let right_wheel = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(4.0, 4.0)),
            color: Color::rgb(0.2, 0.2, 0.2),
            ..Default::default()
        },
        transform: Transform::from_xyz(8.0, -5.0, 0.0),
        ..Default::default()
    };

    move |builder: &mut ChildBuilder| {
        builder.spawn(main_body).with_children(|parent| {
            parent.spawn(top_part);
            parent.spawn(bottom_part);
            parent.spawn(left_wheel);
            parent.spawn(right_wheel);
        });
    }
}

pub fn train_agent_system(
    mut train_agent_query: Query<(&mut TrainAgent, &mut Transform)>,
    node_query: Query<(&Node, &Transform), Without<TrainAgent>>,
    app_resource: Res<AppResource>,
    projection: Res<super::Projection>,
    time: Res<Time>,
) {
    if let Some(ref railway_graph) = app_resource.graph {
        for (mut train_agent, mut transform) in train_agent_query.iter_mut() {
            if let Some(current_node_id) = train_agent.current_node_id {
                update_train_target(&mut train_agent, &railway_graph);
                if let Some(target_node_id) = train_agent.target_node_id {
                    if current_node_id == target_node_id {
                        train_agent.target_node_id = None;
                    } else {
                        if let Some(path) =
                            railway_graph.shortest_path_nodes(current_node_id, target_node_id)
                        {
                            if !path.is_empty() {
                                train_agent.current_node_id = Some(path[1]);
                                if path.len() == 2 {
                                    train_agent.target_node_id = None;
                                } else {
                                    if let Some((_, target_node_transform)) =
                                        node_query.iter().find(|(node, _)| {
                                            node.id == train_agent.target_node_id.unwrap()
                                        })
                                    {
                                        transform.translation = target_node_transform.translation;
                                    }
                                }
                            }
                        }
                    }
                }
                let current_edge = train_agent.current_edge.clone();
                if let Some(edge) = current_edge {
                    let edge_progress =
                        train_agent.edge_progress + train_agent.speed * time.delta_seconds() as f64;
                    update_train_position(
                        &mut train_agent,
                        &mut transform,
                        &edge,
                        time.delta_seconds() as f64,
                        &projection,
                    );
                    train_agent.edge_progress = edge_progress;
                } else if let Some(target_node_id) = train_agent.target_node_id {
                    if let Some(edge) = railway_graph.railway_edge(current_node_id, target_node_id)
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
    if train_agent.target_node_id.is_none() {
        let reachable_nodes = railway_graph.reachable_nodes(train_agent.current_node_id.unwrap());
        if !reachable_nodes.is_empty() {
            let mut rng = rand::thread_rng();
            train_agent.target_node_id = Some(*reachable_nodes.choose(&mut rng).unwrap());
        }
    } else if let Some(target_node_id) = train_agent.target_node_id {
        if train_agent.current_node_id.unwrap() == target_node_id {
            train_agent.target_node_id = None;
            train_agent.current_edge = None; // Reset the current edge when the target node is reached
        } else {
            if let Some(path) = railway_graph
                .shortest_path_nodes(train_agent.current_node_id.unwrap(), target_node_id)
            {
                if !path.is_empty() {
                    train_agent.current_node_id = Some(path[1]);
                    if path.len() == 2 {
                        train_agent.target_node_id = None;
                    }
                }
            }
            // Set the current edge if it's not already set
            if train_agent.current_edge.is_none() {
                if let Some(edge) =
                    railway_graph.railway_edge(train_agent.current_node_id.unwrap(), target_node_id)
                {
                    train_agent.current_edge = Some(edge.clone());
                    train_agent.edge_progress = 0.0;
                }
            }
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
) {
    if let (Some(start_coord), Some(end_coord)) =
        (edge.path.coords().next(), edge.path.coords().last())
    {
        let edge_length = edge.length;

        let distance = train_agent.speed * time_delta;
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
            if let Some(target_node_id) = train_agent.target_node_id {
                train_agent.current_node_id = Some(target_node_id);
                train_agent.target_node_id = None;
                train_agent.current_edge = None;
                train_agent.edge_progress = 0.0;
            }
        }
    }
}
