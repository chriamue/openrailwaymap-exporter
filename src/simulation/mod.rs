//! This module contains types related to railway simulations.
//!
//! A railway simulation consists of a railway graph representing the infrastructure and a
//! list of movable railway objects, such as trains, within the simulation. The module
//! provides a `Simulation` struct to manage the state of the simulation.

use crate::{
    prelude::RailwayGraph,
    railway_objects::{GeoLocation, Movable, NextTarget, RailwayObject},
    types::RailwayObjectId,
};
use std::collections::HashMap;
use std::time::Duration;

use self::agents::{DecisionAgent, RailMovableAction};

pub mod agents;

pub mod environment;
pub use environment::SimulationEnvironment;
use geo::coord;

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
    elapsed_time: Duration,
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
        Self {
            environment: SimulationEnvironment {
                graph,
                objects: HashMap::new(),
            },
            object_agents: HashMap::new(),
            elapsed_time: Duration::default(),
        }
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
            if let Some(agent) = agent {
                self.object_agents.insert(object.id(), agent);
            }
            e.insert(object);
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

    /// Updates the simulation state based on the given delta time.
    ///
    /// # Arguments
    ///
    /// * `delta_time` - The elapsed time since the last update.
    pub fn update(&mut self, delta_time: Duration) {
        // Update the total elapsed time.
        self.elapsed_time += delta_time;

        // Create a copy of the object keys to avoid borrowing `self.objects` mutably while iterating.
        let object_ids: Vec<_> = self.environment.objects.keys().cloned().collect();

        // Iterate over each object in the simulation and update its state based on the delta time.
        for id in object_ids {
            self.update_object(delta_time, id);
        }
    }

    /// Updates the state of the object with the given id based on the given delta time.
    ///
    /// # Arguments
    ///
    /// * `delta_time` - The elapsed time since the last update.
    /// * `id` - The unique identifier of the moveable railway object to be updated.
    fn update_object(&mut self, delta_time: Duration, id: RailwayObjectId) {
        // Get the agent.
        if let Some(agent) = self.object_agents.get_mut(&id) {
            // Observe the environment.
            agent.observe(&self.environment);
        }
        // Get a mutable reference to the object to be updated.
        if let Some(object) = self.environment.objects.get_mut(&id) {
            // Get the action from the decision agent.
            if let Some(agent) = self.object_agents.get_mut(&id) {
                let action = agent.next_action(Some(Duration::from_secs(1)));
                println!("{:?}", action);
                // Update the acceleration based on the action.
                match action {
                    RailMovableAction::Stop => {
                        object.set_acceleration(0.0);
                    }
                    RailMovableAction::AccelerateForward { acceleration } => {
                        object.set_acceleration(acceleration as f64);
                    }
                    RailMovableAction::AccelerateBackward { acceleration } => {
                        object.set_acceleration(-acceleration as f64);
                    }
                }

                // Update speed based on the acceleration
                object.set_speed(object.speed() + delta_time.as_secs_f64() * object.acceleration());
            }
        }
        self.update_object_position(id, delta_time);
    }

    fn update_object_position(&mut self, id: RailwayObjectId, delta_time: Duration) {
        if let Some(object) = self.environment.objects.get_mut(&id) {
            if let Some(current_position) = object.position() {
                let current_speed = object.speed();
                let target = object.next_target();
                let current_location = object.geo_location().unwrap();
    
                let graph = &self.environment.graph;
                let next_node = graph.get_next_node(current_position, target.unwrap_or_default());
    
                if let Some(next_node) = next_node {
                    let edge = graph
                        .railway_edge(current_position, next_node)
                        .expect("Invalid edge");
                    let direction_node = &graph.graph[*graph.node_indices.get(&next_node).unwrap()];
    
                    let direction_coord = coord! { x: direction_node.lon, y: direction_node.lat };
                    let distance_to_travel = current_speed * delta_time.as_secs_f64();
                    let new_geo_location = edge.position_on_edge(current_location, distance_to_travel, direction_coord);
    
                    object.set_geo_location(Some(new_geo_location));
                }
            }
        }
    }
    
}
