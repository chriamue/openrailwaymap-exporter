use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingEvent};

use crate::app3d::train_agent::{self, TrainAgent};

use super::{AppResource, InteractionMode, InteractionModeResource};

/// Represents a node in the railway graph.
#[derive(Component)]
pub struct Node {
    pub id: i64,
}

/// Keeps track of the currently selected start and end nodes.
#[derive(Default, Resource)]
pub struct SelectedNode {
    pub start_node_id: Option<i64>,
    pub end_node_id: Option<i64>,
}

#[allow(clippy::too_many_arguments)]
pub fn select_node_system(
    mut events: EventReader<PickingEvent>,
    app_resource: Res<AppResource>,
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
