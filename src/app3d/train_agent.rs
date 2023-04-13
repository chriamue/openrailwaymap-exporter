// train_agent.rs
use bevy::prelude::*;

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
