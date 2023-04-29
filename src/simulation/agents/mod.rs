//! This module contains types and traits related to decision agents.
//!
//! Decision agents are responsible for choosing the best action based on the current
//! state of the simulation. They interact with movable objects on rail tracks, such as trains,
//! to control their movement.
use std::any::Any;
use std::fmt;
use std::time::Duration;
pub mod decision_agent_factory;
mod forward_until_target_agent;
use super::SimulationEnvironment;
pub use forward_until_target_agent::ForwardUntilTargetAgent;

#[cfg(feature = "ai")]
pub use crate::ai::TrainAgentAI;

/// Represents the possible actions a movable object on rail tracks can take in the simulation.
#[derive(PartialEq, Eq, Hash, Clone, Debug, Default)]
pub enum RailMovableAction {
    /// Bring the movable object to a stop.
    #[default]
    Stop,

    /// Accelerate the movable object forward with a specified acceleration.
    AccelerateForward {
        /// Acceleration in millimeters per second squared (mm/s²).
        acceleration: i32,
    },

    /// Accelerate the movable object backward with a specified acceleration.
    AccelerateBackward {
        /// Acceleration in millimeters per second squared (mm/s²).
        acceleration: i32,
    },
}

impl fmt::Display for RailMovableAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RailMovableAction::AccelerateForward { .. } => write!(f, "AccelerateForward"),
            RailMovableAction::AccelerateBackward { .. } => write!(f, "AccelerateBackward"),
            RailMovableAction::Stop => write!(f, "Stop"),
        }
    }
}

/// A trait that represents a decision agent responsible for choosing the best action
/// based on the current state of the simulation.
pub trait DecisionAgent: Send + Any {
    /// The associated action type for this decision agent.
    type A;

    /// Returns the best action based on the current state of the simulation.
    ///
    /// # Returns
    ///
    /// * `Self::A` - The action chosen by the decision agent.
    fn next_action(&self, delta_time: Option<Duration>) -> Self::A;

    /// Observes the current environment and updates the agent's internal state.
    ///
    /// # Arguments
    ///
    /// * `environment` - The current environment.
    fn observe(&mut self, environment: &SimulationEnvironment);

    /// Returns a reference to the `Any` trait for this object.
    ///
    /// This method is useful for downcasting the object to a concrete type
    /// when working with trait objects.
    ///
    /// # Returns
    ///
    /// A reference to the `Any` trait for the object.
    ///
    fn as_any(&self) -> &dyn Any;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rail_moveable_actions() {
        let actions = vec![
            RailMovableAction::Stop,
            RailMovableAction::AccelerateForward { acceleration: 50 },
            RailMovableAction::AccelerateBackward { acceleration: 20 },
        ];

        if let RailMovableAction::Stop = actions[0] {
            assert!(true);
        } else {
            assert!(false, "Expected Stop action");
        }

        if let RailMovableAction::AccelerateForward { acceleration } = actions[1] {
            assert_eq!(acceleration, 50);
        } else {
            assert!(false, "Expected AccelerateForward action");
        }

        if let RailMovableAction::AccelerateBackward { acceleration } = actions[2] {
            assert_eq!(acceleration, 20);
        } else {
            assert!(false, "Expected AccelerateBackward action");
        }
    }
}
