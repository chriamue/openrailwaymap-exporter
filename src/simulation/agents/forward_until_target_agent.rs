use crate::{
    simulation::{environment::ObservableEnvironment, SimulationEnvironment},
    types::RailwayObjectId,
};

use super::{DecisionAgent, RailMovableAction};

use std::time::Duration;

/// The `ForwardUntilTargetAgent` struct represents a decision agent that moves
/// a railway object forward until it reaches its next target node.
///
/// # Type parameters
///
/// * `T`: A type implementing the `RailwayObject`, `Movable`, and `NextTarget` traits.
#[derive(Debug, Default)]
pub struct ForwardUntilTargetAgent {
    object_id: i64,
    position: Option<i64>,
    target: Option<i64>,
}

impl ForwardUntilTargetAgent {
    /// Constructs a new `ForwardUntilTargetAgent` with the given railway object and `RailwayGraph`.
    ///
    /// # Arguments
    ///
    /// * `object`: The railway object that the agent controls.
    pub fn new(object_id: RailwayObjectId) -> Self {
        Self {
            object_id,
            ..Default::default()
        }
    }

    /// Checks if the railway object has reached its next target node.
    ///
    /// # Returns
    ///
    /// `true` if the railway object's position matches its next target node, `false` otherwise.
    fn target_reached(&self) -> bool {
        self.position == self.target
    }
}

impl DecisionAgent for ForwardUntilTargetAgent {
    type A = RailMovableAction;

    fn next_action(&self, _delta_time: Option<Duration>) -> Self::A {
        if self.target_reached() {
            RailMovableAction::Stop
        } else {
            RailMovableAction::AccelerateForward { acceleration: 20 }
        }
    }

    fn observe(&mut self, environment: &SimulationEnvironment) {
        if let Some(object) = environment
            .get_objects()
            .iter()
            .find(|o| o.id() == self.object_id)
        {
            self.position = object.position();
            self.target = object.next_target();
        }
    }
}

#[cfg(test)]
mod tests {
    use geo::coord;
    use std::collections::VecDeque;

    use super::*;
    use crate::railway_objects::{RailwayObject, Train};

    #[test]
    fn test_decision_agent() {
        let mut train = Train {
            id: 1,
            position: Some(3),
            geo_location: Some(coord! { x:0.0, y: 0.0}),
            next_target: Some(5),
            targets: VecDeque::from(vec![5, 10, 15]),
            ..Default::default()
        };

        // Create agent
        let mut agent = ForwardUntilTargetAgent::new(train.id());

        agent.position = Some(0);
        agent.target = Some(5);
        // Check if the agent suggests the correct action when the target is not reached
        assert_eq!(
            agent.next_action(None),
            RailMovableAction::AccelerateForward { acceleration: 20 }
        );

        // Update agent's object
        train.position = Some(5);
        let agent = ForwardUntilTargetAgent::new(train.id());

        // Check if the agent suggests the correct action when the target is reached
        assert_eq!(agent.next_action(None), RailMovableAction::Stop);
    }
}
