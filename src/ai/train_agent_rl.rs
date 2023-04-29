use crate::ai::TrainAgentState;
use crate::simulation::agents::RailMovableAction;
use rurel::mdp::{Agent, State};

/// Reinforcement Learning Agent for controlling a train in the simulation.
#[derive(Default, Clone, Debug)]
pub struct TrainAgentRL {
    /// The current state of the train agent.
    pub state: TrainAgentState,
    /// The maximum speed the train agent can reach in millimeters per second (mm/s).
    pub max_speed_mm_s: i32,
}

impl TrainAgentRL {
    const TIME_DELTA_MS: u32 = 1000;
}

impl Agent<TrainAgentState> for TrainAgentRL {
    fn current_state(&self) -> &TrainAgentState {
        &self.state
    }

    fn take_action(&mut self, action: &RailMovableAction) {
        match action {
            RailMovableAction::Stop => {
                self.state.current_speed_mm_s = 0;
            }
            RailMovableAction::AccelerateForward { acceleration } => {
                self.state.current_speed_mm_s += acceleration * Self::TIME_DELTA_MS as i32 / 1000;
                self.state.delta_distance_mm =
                    self.state.current_speed_mm_s * Self::TIME_DELTA_MS as i32 / 1000;
            }
            RailMovableAction::AccelerateBackward { acceleration } => {
                self.state.current_speed_mm_s -= acceleration * Self::TIME_DELTA_MS as i32 / 1000;
                self.state.delta_distance_mm =
                    self.state.current_speed_mm_s * Self::TIME_DELTA_MS as i32 / 1000;
            }
        }
        self.state.max_speed_percentage =
            (((self.state.current_speed_mm_s as f64 / self.max_speed_mm_s as f64) * 100.0) as i32)
                .abs();
    }

    fn pick_random_action(&mut self) -> <TrainAgentState as State>::A {
        let action = self.current_state().random_action();
        self.take_action(&action);
        action
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_action() {
        let mut agent = TrainAgentRL {
            state: TrainAgentState {
                delta_distance_mm: 1000,
                current_speed_mm_s: 0,
                max_speed_percentage: 0,
            },
            max_speed_mm_s: (1000.0 * 160.0 / 3.6) as i32,
        };

        // Test AccelerateForward action
        agent.take_action(&RailMovableAction::AccelerateForward { acceleration: 1000 });
        assert_eq!(agent.state.current_speed_mm_s, 1000);
        assert_eq!(agent.state.delta_distance_mm, 1000);

        // Test AccelerateForward action
        agent.take_action(&RailMovableAction::AccelerateForward { acceleration: 500 });
        assert_eq!(agent.state.current_speed_mm_s, 1500);
        assert_eq!(agent.state.delta_distance_mm, 1500);

        // Test AccelerateBackward action
        agent.take_action(&RailMovableAction::AccelerateBackward { acceleration: 500 });
        assert_eq!(agent.state.current_speed_mm_s, 1000);
        assert_eq!(agent.state.delta_distance_mm, 1000);

        // Test Stop action
        agent.take_action(&RailMovableAction::Stop);
        assert_eq!(agent.state.current_speed_mm_s, 0);
    }
}
