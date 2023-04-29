use crate::simulation::agents::RailMovableAction;
use rurel::mdp::State;

/// Represents the state of a train agent in the simulation.
#[derive(PartialEq, Eq, Hash, Clone, Debug, Default)]
pub struct TrainAgentState {
    /// The remaining distance in millimeters the train agent needs to travel.
    pub delta_distance_mm: i32,
    /// The current speed of the train agent in millimeters per second (mm/s).
    pub current_speed_mm_s: i32,
    /// The maximum speed percentage the train agent can reach (e.g., 100 for 100% of the maximum speed).
    pub max_speed_percentage: i32,
}

impl TrainAgentState {
    const MAX_ACCELERATION: i32 = 1000; // 1000 mm/s², approximately 1 m/s²
    const ACCELERATION_STEP: i32 = 20;

    fn speed_reward(&self) -> f64 {
        (self.max_speed_percentage as f64 / 100.0).powi(2)
    }

    fn distance_reward(&self) -> f64 {
        self.delta_distance_mm as f64
    }
}

impl State for TrainAgentState {
    type A = RailMovableAction;

    fn reward(&self) -> f64 {
        20.0 * self.speed_reward() + self.distance_reward()
    }

    fn actions(&self) -> Vec<Self::A> {
        let mut actions = vec![Self::A::Stop];
        for acceleration in 1..=(Self::MAX_ACCELERATION / Self::ACCELERATION_STEP) {
            actions.push(Self::A::AccelerateForward {
                acceleration: acceleration * Self::ACCELERATION_STEP,
            });
            actions.push(Self::A::AccelerateBackward {
                acceleration: acceleration * Self::ACCELERATION_STEP,
            });
        }
        actions
    }

    fn random_action(&self) -> Self::A {
        let actions = self.actions();
        let a_t = rand::random::<usize>() % actions.len();
        actions[a_t].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_train_agent_state() {
        let state = TrainAgentState {
            delta_distance_mm: 1000,
            current_speed_mm_s: 0,
            max_speed_percentage: 0,
        };

        assert_eq!(state.delta_distance_mm, 1000);
        assert_eq!(state.current_speed_mm_s, 0);
        assert_eq!(state.max_speed_percentage, 0);
    }
}
