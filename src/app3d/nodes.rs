use bevy::prelude::*;

use crate::{
    app3d::train_agent::{self, create_new_train_id, create_train, TrainAgent},
    prelude::RailwayGraphExt,
    types::NodeId,
};
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_mod_picking::prelude::Pointer;
use bevy_mod_picking::{prelude::Click, PickableBundle};

use super::{AppResource, InteractionMode, InteractionModeResource};

/// Represents a node in the railway graph.
#[derive(Component)]
pub struct Node {
    pub id: NodeId,
}

/// Keeps track of the currently selected start and end nodes.
#[derive(Default, Resource)]
pub struct SelectedNode {
    pub start_node_id: Option<NodeId>,
    pub end_node_id: Option<NodeId>,
}

#[derive(Debug, Component, Event)]
pub struct NodeSelectedEvent(Entity);

impl From<ListenerInput<Pointer<Click>>> for NodeSelectedEvent {
    fn from(click_event: ListenerInput<Pointer<Click>>) -> Self {
        Self(click_event.target)
    }
}

#[allow(clippy::too_many_arguments)]
pub fn select_node_system(
    mut events: EventReader<NodeSelectedEvent>,
    app_resource: Res<AppResource>,
    mut selected_node: ResMut<SelectedNode>,
    q_node: Query<(Entity, &Node, &Transform), Without<Camera>>,
    interaction_mode: Res<InteractionModeResource>,
    mut commands: Commands,
    materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut selection = None;
    for select_event in events.iter() {
        if let Ok((entity, node, transform)) = q_node.get(select_event.0) {
            selection = Some((entity, node.id, *transform));
        }
    }

    if let Some((entity, node_id, transform)) = selection {
        // Check the current interaction mode
        match interaction_mode.mode {
            InteractionMode::SelectMode => {
                selected_node.end_node_id = selected_node.start_node_id;
                selected_node.start_node_id = Some(node_id);
                if let Some(graph) = &app_resource.graph {
                    let edges = graph.get_edges_of_node(node_id);
                    println!(
                        "Selected node: {:?} {:?} {:?}",
                        entity,
                        graph.get_node_by_id(node_id),
                        edges
                            .iter()
                            .map(|e| (e.id, e.source, e.target))
                            .collect::<Vec<_>>()
                    );
                }
            }
            InteractionMode::PlaceTrain => {
                if let Some(simulation) = &app_resource.simulation {
                    let mut sim = simulation.write().unwrap();
                    let id = create_new_train_id();
                    let id = create_train(id, Some(node_id), None, &mut sim);
                    let train_agent = TrainAgent::new(id);

                    println!("Placing train on node: {:?}", node_id);
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
                        .insert(PickableBundle::default())
                        .with_children(train_agent::create_train_agent_bundle(
                            materials,
                            asset_server,
                        ));
                }
            }
        }
    }
}
