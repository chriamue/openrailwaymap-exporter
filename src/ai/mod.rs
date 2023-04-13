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

/// Represents the state of a train agent in the simulation.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct TrainAgentState {
    /// The remaining distance in meters the train agent needs to travel.
    pub remaining_distance_m: i32,
    /// The current speed of the train agent in kilometers per hour.
    pub current_speed_in_kmh: i32,
    /// The maximum speed the train agent can reach in kilometers per hour.
    pub max_speed_in_kmh: i32,
}

/// Represents the possible actions a train agent can take in the simulation.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum TrainAgentAction {
    /// Stop the train agent.
    Stop,
    /// Drive the train agent forward at the specified speed in kilometers per hour.
    DriveForward {
        /// Speed the train should travel
        speed: i32,
    },
    /// Drive the train agent backward at the specified speed in kilometers per hour.
    DriveBackward {
        /// Speed the train should travel
        speed: i32,
    },
}

impl State for TrainAgentState {
    type A = TrainAgentAction;

    fn reward(&self) -> f64 {
        -((self.max_speed_in_kmh as f64 / self.current_speed_in_kmh as f64) as f64
             + (10000.0 / (1.0 + self.remaining_distance_m as f64)) / 10000.0)
    }

    fn actions(&self) -> Vec<TrainAgentAction> {
        let mut actions = vec![TrainAgentAction::Stop];
        for speed in 1..=(self.max_speed_in_kmh) {
            actions.push(TrainAgentAction::DriveForward { speed });
            actions.push(TrainAgentAction::DriveBackward { speed });
        }
        actions
    }
}

/// A reinforcement learning agent that controls a train in the simulation.
pub struct TrainAgentRL {
    /// The railway graph representing the train network.
    pub railway_graph: RailwayGraph,
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
                self.state.current_speed_in_kmh = 0;
            }
            TrainAgentAction::DriveForward { speed } => {
                self.state.remaining_distance_m -= (self.state.current_speed_in_kmh as f32 / 3.6) as i32;
                self.state.current_speed_in_kmh = *speed;
                
            }
            TrainAgentAction::DriveBackward { speed } => {
                self.state.remaining_distance_m += (self.state.current_speed_in_kmh as f32 / 3.6) as i32;
                self.state.current_speed_in_kmh = *speed;
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
        remaining_distance_m: 1000,
        current_speed_in_kmh: 0,
        max_speed_in_kmh: 160,
    };
    let mut trainer = AgentTrainer::new();
    let mut agent = TrainAgentRL {
        railway_graph,
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
            remaining_distance_m: 1000,
            current_speed_in_kmh: 0,
            max_speed_in_kmh: 160,
        };

        assert_eq!(state.remaining_distance_m, 1000);
        assert_eq!(state.current_speed_in_kmh, 0);
        assert_eq!(state.max_speed_in_kmh, 160);
    }

    #[test]
    fn test_train_agent_actions() {
        let actions = vec![
            TrainAgentAction::Stop,
            TrainAgentAction::DriveForward { speed: 50 },
            TrainAgentAction::DriveBackward { speed: 20 },
        ];

        if let TrainAgentAction::Stop = actions[0] {
            assert!(true);
        } else {
            assert!(false, "Expected Stop action");
        }

        if let TrainAgentAction::DriveForward { speed } = actions[1] {
            assert_eq!(speed, 50);
        } else {
            assert!(false, "Expected DriveForward action");
        }

        if let TrainAgentAction::DriveBackward { speed } = actions[2] {
            assert_eq!(speed, 20);
        } else {
            assert!(false, "Expected DriveBackward action");
        }
    }
}
