//! This module contains types related to railway simulations.
//!
//! A railway simulation consists of a railway graph representing the infrastructure and a
//! list of movable railway objects, such as trains, within the simulation. The module
//! provides a `Simulation` struct to manage the state of the simulation.

use crate::{
    prelude::RailwayGraph,
    railway_objects::{Movable, RailwayObject},
};
use std::collections::HashMap;
use std::time::Duration;

use self::agents::{DecisionAgent, RailMovableAction};

pub mod agents;

#[cfg(test)]
mod tests;

/// A trait that defines an object within the simulation that can move along a railway.
pub trait SimulationObject: RailwayObject + Movable {}
impl<T: RailwayObject + Movable> SimulationObject for T {}

/// A `Simulation` struct holding a railway graph and a list of moveable railway objects.
#[derive(Debug, Clone)]
pub struct Simulation<SO, DA>
where
    SO: SimulationObject,
    DA: DecisionAgent<A = RailMovableAction>,
{
    /// The railway graph representing the railway infrastructure.
    pub graph: RailwayGraph,

    /// A list of moveable railway objects within the simulation.
    pub objects: HashMap<i64, SO>,
    /// A list of agents
    pub object_agents: HashMap<i64, DA>,
    elapsed_time: Duration,
}

impl<SO, DA> Simulation<SO, DA>
where
    SO: SimulationObject,
    DA: DecisionAgent<A = RailMovableAction>,
{
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
            graph,
            objects: HashMap::new(),
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
    pub fn add_object(&mut self, object: SO, agent: Option<DA>) -> bool {
        if let std::collections::hash_map::Entry::Vacant(e) = self.objects.entry(object.id()) {
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
        self.objects.remove(&id).is_some()
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
        let object_ids: Vec<_> = self.objects.keys().cloned().collect();

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
    fn update_object(&mut self, delta_time: Duration, id: i64) {
        // Get a mutable reference to the object to be updated.
        if let Some(object) = self.objects.get_mut(&id) {
            // Get the action from the decision agent.
            if let Some(agent) = self.object_agents.get(&id) {
                let action = agent.next_action();

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
    }
}
