//! This module contains types related to railway simulations.
//!
//! A railway simulation consists of a railway graph representing the infrastructure and a
//! list of movable railway objects, such as trains, within the simulation. The module
//! provides a `Simulation` struct to manage the state of the simulation.

use std::collections::HashMap;

use crate::{
    prelude::RailwayGraph,
    railway_objects::{Moveable, RailwayObject},
};

pub mod agents;

/// A `Simulation` struct holding a railway graph and a list of moveable railway objects.
#[derive(Debug, Clone)]
pub struct Simulation<T>
where
    T: RailwayObject + Moveable,
{
    /// The railway graph representing the railway infrastructure.
    pub graph: RailwayGraph,

    /// A list of moveable railway objects within the simulation.
    pub objects: HashMap<i64, T>,
}

impl<T> Simulation<T>
where
    T: RailwayObject + Moveable,
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
    pub fn add_object(&mut self, object: T) -> bool {
        if self.objects.contains_key(&object.id()) {
            false
        } else {
            self.objects.insert(object.id(), object);
            true
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
}
