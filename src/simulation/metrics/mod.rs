//! This module contains types related to handling and processing metrics during railway simulations.
//!
//! Metrics handlers are used to process and collect metrics from simulation events. The module provides a `MetricsHandler`
//! trait that defines the interface for handling simulation events and extracting metric values.
//!
//! The `ActionCountHandler` and `TargetReachedHandler` structs are provided as example implementations of metrics handlers.
//! `ActionCountHandler` counts the number of `RailMovableAction` events, while `TargetReachedHandler` counts the number
//! of target reached events during the simulation.
//!
//! Handlers can be registered with the `Simulation` struct to process events during the simulation execution.
use crate::simulation::events::SimulationEvent;
use std::any::Any;
mod action_count_handler;
pub use action_count_handler::ActionCountHandler;

mod target_reached_handler;
pub use target_reached_handler::TargetReachedHandler;

/// The `MetricsHandler` trait defines the interface for handling simulation events and extracting metric values.
///
/// Implementors of this trait should process the events and update their internal state to compute the desired metric.
pub trait MetricsHandler: Send {
    /// Handles a simulation event.
    ///
    /// # Arguments
    ///
    /// * `event` - A reference to a simulation event.
    fn handle(&mut self, event: &dyn SimulationEvent);

    /// Returns the current value of the metric.
    ///
    /// # Returns
    ///
    /// A `f64` representing the current value of the metric.
    fn get_value(&self) -> f64;

    /// Returns a reference to the underlying Any type for downcasting.
    ///
    /// # Returns
    ///
    /// A reference to the underlying Any type.
    fn as_any(&self) -> &dyn Any;
}
