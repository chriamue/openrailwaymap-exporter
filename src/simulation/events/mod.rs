//! This module contains types related to simulation events in railway simulations.
//!
//! Simulation events are used to represent specific occurrences during the execution of a simulation, such as a train
//! changing its action or reaching a target.
//!
//! The module provides a `SimulationEvent` trait that defines the interface for all simulation events. The
//! `RailMovableEvent` and `TargetReachedEvent` structs are example implementations of this trait, representing a
//! change in a `RailMovableAction` and a target being reached, respectively.

use crate::simulation::agents::RailMovableAction;
use std::any::Any;

/// The `SimulationEvent` trait defines the interface for all simulation events.
pub trait SimulationEvent {
    /// Returns a reference to the event as a trait object implementing `Any`.
    /// This method is useful for downcasting the event to a concrete type.
    fn as_any(&self) -> &dyn Any;
}

/// A `RailMovableEvent` represents a change in a `RailMovableAction` during a simulation.
pub struct RailMovableEvent {
    /// The `RailMovableAction` associated
    pub action: RailMovableAction,
}

impl SimulationEvent for RailMovableEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A `TargetReachedEvent` represents a target being reached during a simulation.
pub struct TargetReachedEvent {}

impl SimulationEvent for TargetReachedEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rail_movable_event_as_any() {
        let event = RailMovableEvent {
            action: RailMovableAction::Stop,
        };

        assert!(event.as_any().is::<RailMovableEvent>());
    }

    #[test]
    fn target_reached_event_as_any() {
        let event = TargetReachedEvent {};

        assert!(event.as_any().is::<TargetReachedEvent>());
    }
}
