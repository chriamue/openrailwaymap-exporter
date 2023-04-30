//! This module contains types related to railway simulations.
//!
//! A railway simulation consists of a railway graph representing the infrastructure and a
//! list of movable railway objects, such as trains, within the simulation. The module
//! provides a `Simulation` struct to manage the state of the simulation.

use self::{
    agents::{DecisionAgent, RailMovableAction},
    environment::{ObservableEnvironment, ObservableEnvironmentRef},
};
use crate::{
    algorithms::is_middle_coord_between,
    prelude::RailwayGraph,
    railway_objects::{GeoLocation, Movable, NextTarget, RailwayObject, Train},
    types::RailwayObjectId,
};
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;
pub mod agents;

pub mod environment;
use bevy::prelude::warn;
pub use environment::SimulationEnvironment;
use geo::coord;
use rand::seq::SliceRandom;
use uom::si::{
    acceleration::{meter_per_second_squared, Acceleration},
    f64::{Length, Time},
    length::meter,
    time::second,
};
mod simulation_executor;
use crate::simulation::events::{RailMovableEvent, SimulationEvent, TargetReachedEvent};
use crate::simulation::metrics::{ActionCountHandler, MetricsHandler, TargetReachedHandler};
pub use simulation_executor::SimulationExecutor;

pub mod commands;
pub mod events;
pub mod metrics;
#[cfg(test)]
mod tests;

/// A trait that defines an object within the simulation that can move along a railway.
pub trait SimulationObject:
    RailwayObject + Movable + NextTarget + GeoLocation + Send + Sync
{
}
impl<T: RailwayObject + Movable + NextTarget + GeoLocation + Send + Sync> SimulationObject for T {}

/// A `Simulation` struct holding a railway graph and a list of moveable railway objects.
pub struct Simulation {
    /// The simulation environment
    pub environment: SimulationEnvironment,
    /// A list of agents
    pub object_agents: HashMap<RailwayObjectId, Box<dyn DecisionAgent<A = RailMovableAction>>>,
    /// A list of metrics handlers
    pub metrics_handlers: Vec<Box<dyn MetricsHandler>>,
    /// Elapsed time of simulation
    elapsed_time: Duration,
    /// simulation pause state
    pub is_paused: bool,
}

impl fmt::Debug for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Simulation")
            .field("railway_objects", &self.environment.objects.len())
            .finish()
    }
}

impl Simulation {
    /// Creates a new simulation with the given railway graph.
    ///
    /// # Arguments
    ///
    /// * `graph` - The railway graph representing the railway infrastructure.
    ///
    /// # Returns
    ///
    /// A new `Simulation` instance.
    ///
    pub fn new(graph: RailwayGraph) -> Self {
        let mut default_metrics_handler: Vec<Box<dyn MetricsHandler>> = vec![];
        default_metrics_handler.push(Box::new(ActionCountHandler::new()));
        default_metrics_handler.push(Box::new(TargetReachedHandler::new()));

        Self {
            environment: SimulationEnvironment {
                graph,
                objects: HashMap::new(),
            },
            object_agents: HashMap::new(),
            metrics_handlers: default_metrics_handler,
            elapsed_time: Duration::default(),
            is_paused: false,
        }
    }

    /// Returns a reference to the observable environment of the simulation.
    ///
    /// The observable environment allows external components to access the
    /// state of the simulation without being able to modify it. This is useful
    /// for agents to observe the simulation state and make decisions based on it.
    ///
    /// # Returns
    ///
    /// A reference to a trait object implementing the `ObservableEnvironment` trait,
    /// which provides read-only access to the simulation environment.
    ///
    pub fn get_observable_environment(&self) -> &(dyn ObservableEnvironment + 'static) {
        self.environment.as_observable_env()
    }

    /// Adds a moveable railway object to the simulation.
    ///
    /// # Arguments
    ///
    /// * `object` - The moveable railway object to be added to the simulation.
    ///
    /// # Returns
    ///
    /// A boolean indicating if the object was successfully added.
    ///
    pub fn add_object(
        &mut self,
        object: Box<dyn SimulationObject>,
        agent: Option<Box<dyn DecisionAgent<A = RailMovableAction>>>,
    ) -> bool {
        if let std::collections::hash_map::Entry::Vacant(e) =
            self.environment.objects.entry(object.id())
        {
            let id = object.id();
            e.insert(object);

            if let Some(agent) = agent {
                self.add_agent_for_object(id, agent);
            }
            true
        } else {
            false
        }
    }

    /// Removes a moveable railway object from the simulation.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the moveable railway object to be removed.
    ///
    /// # Returns
    ///
    /// A boolean indicating if the object was successfully removed.
    ///
    pub fn remove_object(&mut self, id: i64) -> bool {
        self.environment.objects.remove(&id).is_some()
    }

    /// Adds a decision agent for an object in the simulation.
    ///
    /// # Arguments
    ///
    /// * `object_id` - The unique identifier of the object.
    /// * `agent` - The decision agent to be added.
    ///
    /// # Returns
    ///
    /// A boolean indicating if the agent was successfully added.
    ///
    pub fn add_agent_for_object(
        &mut self,
        object_id: RailwayObjectId,
        agent: Box<dyn DecisionAgent<A = RailMovableAction>>,
    ) -> bool {
        if self.environment.objects.contains_key(&object_id) {
            self.object_agents.insert(object_id, agent);
            true
        } else {
            false
        }
    }

    /// Registers a metrics handler for the simulation.
    ///
    /// This function adds a new metrics handler to the simulation. The metrics handler
    /// will be used to process events and gather metrics during the simulation run.
    ///
    /// # Arguments
    ///
    /// * `handler` - A boxed metrics handler that implements the `MetricsHandler` trait.
    ///
    pub fn register_metrics_handler(&mut self, handler: Box<dyn MetricsHandler>) {
        self.metrics_handlers.push(handler);
    }

    /// Handles a simulation event by passing it to all registered metrics handlers.
    ///
    /// This function is called internally by the simulation engine whenever a simulation
    /// event occurs. It iterates through all registered metrics handlers and calls their
    /// `handle` function with the event as an argument.
    ///
    /// # Arguments
    ///
    /// * `event` - A reference to a simulation event that implements the `SimulationEvent` trait.
    ///
    /// This function is not meant to be called directly by the user. It is called internally
    /// by the simulation engine.
    fn handle_event(&mut self, event: &dyn SimulationEvent) {
        for handler in &mut self.metrics_handlers {
            handler.handle(event);
        }
    }

    /// Updates the simulation state based on the given delta time.
    ///
    /// # Arguments
    ///
    /// * `delta_time` - The elapsed time since the last update.
    pub fn update(&mut self, delta_time: Duration) {
        if !self.is_paused {
            // Update the total elapsed time.
            self.elapsed_time += delta_time;

            // Create a copy of the object keys to avoid borrowing `self.objects` mutably while iterating.
            let object_ids: Vec<_> = self.environment.objects.keys().cloned().collect();

            // Iterate over each object in the simulation and update its state based on the delta time.
            for id in object_ids {
                self.update_object(delta_time, id);
            }
        }
    }

    /// Updates the state of the object with the given id based on the given delta time.
    ///
    /// # Arguments
    ///
    /// * `delta_time` - The elapsed time since the last update.
    /// * `id` - The unique identifier of the moveable railway object to be updated.
    fn update_object(&mut self, delta_time: Duration, id: RailwayObjectId) {
        if let Some(agent) = self.object_agents.get_mut(&id) {
            // Observe the environment.
            agent.observe(&self.environment);
        }
        let mut event = None;
        if let Some(object) = self.environment.objects.get_mut(&id) {
            // Get the action from the decision agent.
            if let Some(agent) = self.object_agents.get(&id) {
                let action = agent.next_action(Some(delta_time));
                // Update the acceleration based on the action.
                match action {
                    RailMovableAction::Stop => {
                        let speed = object.speed();
                        if speed.is_sign_positive() {
                            object.set_acceleration(-object.acceleration().abs());
                        } else if speed.is_sign_negative() {
                            object.set_acceleration(object.acceleration().abs());
                        } else {
                            object.set_acceleration(Acceleration::new::<meter_per_second_squared>(
                                0.0,
                            ));
                        }
                    }
                    RailMovableAction::AccelerateForward { acceleration } => {
                        object.set_acceleration(Acceleration::new::<meter_per_second_squared>(
                            acceleration as f64,
                        ));
                    }
                    RailMovableAction::AccelerateBackward { acceleration } => {
                        object.set_acceleration(Acceleration::new::<meter_per_second_squared>(
                            -acceleration as f64,
                        ));
                    }
                }

                event = Some(RailMovableEvent { action });

                // Update speed based on the acceleration
                object.set_speed(object.max_speed().min(
                    object.speed()
                        + Time::new::<second>(delta_time.as_secs_f64()) * object.acceleration(),
                ));
            }
        }
        if let Some(event) = event {
            self.handle_event(&event);
        }

        self.update_object_position(id, delta_time);
        self.update_train_target(id);
    }

    fn update_object_position(&mut self, id: RailwayObjectId, delta_time: Duration) {
        const NEXT_NODE_DISTANCE_TOLERANCE: f64 = 1.0;
        if let Some(object) = self.environment.objects.get_mut(&id) {
            if let Some(current_position) = object.position() {
                let current_speed = object.speed();
                let target = object.next_target();
                if let Some(current_location) = object.geo_location() {
                    let graph = &self.environment.graph;
                    let next_node =
                        graph.get_next_node(current_position, target.unwrap_or_default());

                    if let Some(next_node_id) = next_node {
                        let edge = graph
                            .railway_edge(current_position, next_node_id)
                            .expect("Invalid edge");
                        let next_node = graph.get_node_by_id(next_node_id).unwrap();

                        let direction_coord = coord! { x: next_node.lon, y: next_node.lat };
                        let distance_to_travel =
                            current_speed * Time::new::<second>(delta_time.as_secs_f64());

                        let new_geo_location = edge.position_on_edge(
                            current_location,
                            distance_to_travel,
                            direction_coord,
                        );

                        let current_node = graph.get_node_by_id(current_position).unwrap();

                        // reached next node
                        if is_middle_coord_between(
                            coord! {x:current_node.lon, y: current_node.lat},
                            coord! {x: next_node.lon, y: next_node.lat},
                            new_geo_location,
                        ) || edge.distance_to_end(new_geo_location, direction_coord)
                            < Length::new::<meter>(NEXT_NODE_DISTANCE_TOLERANCE)
                        {
                            object.set_position(Some(next_node_id));
                        }
                        object.set_geo_location(Some(new_geo_location));
                    }
                } else {
                    warn!("object {} has no coordinates", object.id())
                }
            }
        }
    }

    fn update_train_target(&mut self, id: RailwayObjectId) {
        let mut event = None;
        if let Some(object) = self.environment.objects.get_mut(&id) {
            if let Some(train) = object.as_any_mut().downcast_mut::<Train>() {
                if train.next_target().is_none() || train.position() == train.next_target() {
                    let reachable_nodes = self
                        .environment
                        .graph
                        .reachable_nodes(train.position().unwrap());
                    if !reachable_nodes.is_empty() {
                        event = Some(TargetReachedEvent {});
                        let mut rng = rand::thread_rng();
                        train.set_next_target(Some(*reachable_nodes.choose(&mut rng).unwrap()));
                    }
                }
            }
        }
        if let Some(event) = event {
            self.handle_event(&event);
        }
    }
}
