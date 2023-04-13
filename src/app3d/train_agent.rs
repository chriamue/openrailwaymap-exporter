// train_agent.rs
use bevy::prelude::*;
use rand::seq::SliceRandom;

use super::{AppResource, Node};
use crate::railway_algorithms::PathFinding;
use std::sync::atomic::{AtomicI32, Ordering};

static TRAIN_AGENT_ID: AtomicI32 = AtomicI32::new(0);

#[derive(Component)]
pub struct TrainAgent {
    pub id: i32,
    pub current_node_id: Option<i64>,
    pub target_node_id: Option<i64>,
}

impl TrainAgent {
    pub fn new(id: i32, current_node_id: Option<i64>, target_node_id: Option<i64>) -> Self {
        Self {
            id,
            current_node_id,
            target_node_id,
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
) {
    if let Some(ref railway_graph) = app_resource.graph {
        for (mut train_agent, mut transform) in train_agent_query.iter_mut() {
            if let Some(current_node_id) = train_agent.current_node_id {
                if train_agent.target_node_id.is_none() {
                    let reachable_nodes = railway_graph.reachable_nodes(current_node_id);
                    if !reachable_nodes.is_empty() {
                        let mut rng = rand::thread_rng();
                        train_agent.target_node_id =
                            Some(*reachable_nodes.choose(&mut rng).unwrap());
                    }
                } else if let Some(target_node_id) = train_agent.target_node_id {
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
            }
        }
    }
}
