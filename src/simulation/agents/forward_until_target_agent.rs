use crate::railway_objects::{Movable, NextTarget, RailwayObject};

use super::{DecisionAgent, RailMovableAction};

use std::boxed::Box;

/// The `ForwardUntilTargetAgent` struct represents a decision agent that moves
/// a railway object forward until it reaches its next target node.
///
/// # Type parameters
///
/// * `T`: A type implementing the `RailwayObject`, `Movable`, and `NextTarget` traits.
pub struct ForwardUntilTargetAgent<T>
where
    T: RailwayObject<Node = i64> + Movable + NextTarget,
{
    object: Box<T>,
}

impl<T> ForwardUntilTargetAgent<T>
where
    T: RailwayObject<Node = i64> + Movable + NextTarget,
{
    /// Constructs a new `ForwardUntilTargetAgent` with the given railway object and `RailwayGraph`.
    ///
    /// # Arguments
    ///
    /// * `object`: The railway object that the agent controls.
    pub fn new(object: T) -> Self {
        Self {
            object: Box::new(object),
        }
    }

    /// Checks if the railway object has reached its next target node.
    ///
    /// # Returns
    ///
    /// `true` if the railway object's position matches its next target node, `false` otherwise.
    fn target_reached(&self) -> bool {
        self.object.position() == self.object.next_target()
    }
}

impl<T> DecisionAgent for ForwardUntilTargetAgent<T>
where
    T: RailwayObject<Node = i64> + Movable + NextTarget,
{
    type A = RailMovableAction;

    /// Determines the next action for the railway object based on its current state and the `RailwayGraph`.
    ///
    /// # Returns
    ///
    /// A `RailMovableAction` representing the action the railway object should take.
    fn next_action(&self) -> Self::A {
        if self.target_reached() {
            RailMovableAction::Stop
        } else {
            RailMovableAction::AccelerateForward { acceleration: 20 }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use geo_types::coord;

    use super::*;
    use crate::railway_objects::Train;

    #[test]
    fn test_decision_agent() {
        let mut train = Train {
            id: 1,
            position: Some(0),
            geo_location: Some(coord! { x:0.0, y: 0.0}),
            next_target: Some(5),
            targets: VecDeque::from(vec![5, 10, 15]),
            ..Default::default()
        };

        // Create agent
        let agent = ForwardUntilTargetAgent::new(train.clone());

        // Check if the agent suggests the correct action when the target is not reached
        assert_eq!(
            agent.next_action(),
            RailMovableAction::AccelerateForward { acceleration: 20 }
        );

        // Update agent's object
        train.position = Some(5);
        let agent = ForwardUntilTargetAgent::new(train.clone());

        // Check if the agent suggests the correct action when the target is reached
        assert_eq!(agent.next_action(), RailMovableAction::Stop);
    }
}
