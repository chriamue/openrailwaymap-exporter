//! The `ai` module provides a reinforcement learning agent that controls a train in a railway network simulation.
//!
//! The module defines the `TrainAgentState` struct, which represents the state of a train agent in the simulation,
//! and the `TrainAgentAction` enum, which represents the possible actions a train agent can take in the simulation.
//!
//! The main component of this module is the `TrainAgentRL` struct, which implements the `rurel::mdp::Agent` trait
//! for the train agent. This struct handles the reinforcement learning process, updating the train agent's state
//! based on the actions taken and the rewards received.
//!
use crate::prelude::RailwayGraph;
use rurel::mdp::{Agent, State};
use rurel::strategy::explore::RandomExploration;
use rurel::strategy::learn::QLearning;
use rurel::strategy::terminate::FixedIterations;
use rurel::AgentTrainer;

/// Represents the possible actions a train agent can take in the simulation.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum TrainAgentAction {
    /// Stop the train agent.
    Stop,
    /// Accelerate the train agent forward.
    AccelerateForward {
        /// acceleration in meters per second squared (mm/s²).
        acceleration: i32,
    },
    /// Accelerate the train agent backward.
    AccelerateBackward {
        /// acceleration in meters per second squared (mm/s²).
        acceleration: i32,
    },
}

/// Represents the state of a train agent in the simulation.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct TrainAgentState {
    /// The remaining distance in meters the train agent needs to travel.
    pub remaining_distance_mm: i64,
    /// The current speed of the train agent in millimeters per second (mm/s).
    pub current_speed_mm_s: i32,
    /// The maximum speed the train agent can reach in millimeters per second (mm/s).
    pub max_speed_mm_s: i32,
    /// The time delta between actions in milliseconds.
    pub time_delta_ms: i32,
}

impl TrainAgentState {
    fn speed_reward(&self) -> f64 {
        self.max_speed_mm_s as f64 / self.current_speed_mm_s.abs() as f64
    }

    fn distance_reward(&self) -> f64 {
        10000.0 / (1.0 + self.remaining_distance_mm as f64)
    }
}

impl State for TrainAgentState {
    type A = TrainAgentAction;

    fn reward(&self) -> f64 {
        self.speed_reward() + self.distance_reward() / 10000.0
    }

    fn actions(&self) -> Vec<TrainAgentAction> {
        let max_acceleration = 1000; // 1000 mm/s², approximately 1 m/s²
        let acceleraton_step = 20;
        let mut actions = vec![TrainAgentAction::Stop];
        for acceleration in 1..=max_acceleration / acceleraton_step {
            actions.push(TrainAgentAction::AccelerateForward {
                acceleration: acceleration * acceleraton_step,
            });
            actions.push(TrainAgentAction::AccelerateBackward {
                acceleration: acceleration * acceleraton_step,
            });
        }
        actions
    }
}

/// A reinforcement learning agent that controls a train in the simulation.
pub struct TrainAgentRL {
    /// The railway graph representing the train network.
    pub railway_graph: Option<RailwayGraph>,
    /// The current state of the train agent.
    pub state: TrainAgentState,
}

impl Agent<TrainAgentState> for TrainAgentRL {
    fn current_state(&self) -> &TrainAgentState {
        &self.state
    }

    fn take_action(&mut self, action: &TrainAgentAction) {
        match action {
            TrainAgentAction::Stop => {
                self.state.current_speed_mm_s = 0;
            }
            TrainAgentAction::AccelerateForward { acceleration } => {
                self.state.current_speed_mm_s += acceleration * self.state.time_delta_ms / 1000;
                self.state.remaining_distance_mm -=
                    self.state.current_speed_mm_s as i64 * self.state.time_delta_ms as i64 / 1000;
            }
            TrainAgentAction::AccelerateBackward { acceleration } => {
                self.state.current_speed_mm_s -= acceleration * self.state.time_delta_ms / 1000;
                self.state.remaining_distance_mm +=
                    self.state.current_speed_mm_s as i64 * self.state.time_delta_ms as i64 / 1000;
            }
        }
    }

    fn pick_random_action(&mut self) -> <TrainAgentState as State>::A {
        let action = self.current_state().random_action();
        self.take_action(&action);
        action
    }
}

/// Trains a reinforcement learning agent to control a train in the simulation.
pub fn train(railway_graph: RailwayGraph) {
    let initial_state = TrainAgentState {
        remaining_distance_mm: 1000 * 1000,
        current_speed_mm_s: 0,
        max_speed_mm_s: ((160.0 / 3.6) as i32) * 1000,
        time_delta_ms: 1000,
    };
    let mut trainer = AgentTrainer::new();
    let mut agent = TrainAgentRL {
        railway_graph: Some(railway_graph),
        state: initial_state,
    };
    trainer.train(
        &mut agent,
        &QLearning::new(0.2, 0.01, 2.),
        &mut FixedIterations::new(10000),
        &RandomExploration::new(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_train_agent_state() {
        let state = TrainAgentState {
            remaining_distance_mm: 1000 * 1000,
            current_speed_mm_s: 0,
            max_speed_mm_s: ((160.0 / 3.6) as i32) * 1000,
            time_delta_ms: 1000,
        };

        assert_eq!(state.remaining_distance_mm, 1000000);
        assert_eq!(state.current_speed_mm_s, 0);
        assert_eq!(state.max_speed_mm_s, ((160.0 / 3.6) as i32) * 1000);
        assert_eq!(state.time_delta_ms, 1000);
    }

    #[test]
    fn test_train_agent_actions() {
        let actions = vec![
            TrainAgentAction::Stop,
            TrainAgentAction::AccelerateForward { acceleration: 50 },
            TrainAgentAction::AccelerateBackward { acceleration: 20 },
        ];

        if let TrainAgentAction::Stop = actions[0] {
            assert!(true);
        } else {
            assert!(false, "Expected Stop action");
        }

        if let TrainAgentAction::AccelerateForward { acceleration } = actions[1] {
            assert_eq!(acceleration, 50);
        } else {
            assert!(false, "Expected AccelerateForward action");
        }

        if let TrainAgentAction::AccelerateBackward { acceleration } = actions[2] {
            assert_eq!(acceleration, 20);
        } else {
            assert!(false, "Expected AccelerateBackward action");
        }
    }

    #[test]
    fn test_take_action() {
        let mut agent = TrainAgentRL {
            railway_graph: None,
            state: TrainAgentState {
                remaining_distance_mm: 1000 * 1000,
                current_speed_mm_s: 0,
                max_speed_mm_s: (160.0 / (3.6 * 1000.0)) as i32,
                time_delta_ms: 1000,
            },
        };

        // Test AccelerateForward action
        agent.take_action(&TrainAgentAction::AccelerateForward { acceleration: 1000 });
        assert_eq!(agent.state.current_speed_mm_s, 1000);
        assert_eq!(agent.state.remaining_distance_mm, 1000 * 1000 - 1000);

        // Test AccelerateForward action
        agent.take_action(&TrainAgentAction::AccelerateForward { acceleration: 500 });
        assert_eq!(agent.state.current_speed_mm_s, 1500);
        assert_eq!(agent.state.remaining_distance_mm, 1000 * 1000 - 2500);

        // Test AccelerateBackward action
        agent.take_action(&TrainAgentAction::AccelerateBackward { acceleration: 500 });
        assert_eq!(agent.state.current_speed_mm_s, 1000);
        assert_eq!(agent.state.remaining_distance_mm, 1000 * 1000 - 1500);

        // Test Stop action
        agent.take_action(&TrainAgentAction::Stop);
        assert_eq!(agent.state.current_speed_mm_s, 0);
    }
}
