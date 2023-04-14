//! The `ai` module provides a reinforcement learning agent that controls a train in a railway network simulation.
//!
//! The module defines the `TrainAgentState` struct, which represents the state of a train agent in the simulation,
//! and the `TrainAgentAction` enum, which represents the possible actions a train agent can take in the simulation.
//!
//! The main component of this module is the `TrainAgentRL` struct, which implements the `rurel::mdp::Agent` trait
//! for the train agent. This struct handles the reinforcement learning process, updating the train agent's state
//! based on the actions taken and the rewards received.
//!
use std::fmt;
use std::sync::{Arc, Mutex};

use crate::prelude::RailwayGraph;
use rurel::mdp::{Agent, State};
use rurel::strategy::explore::RandomExploration;
use rurel::strategy::learn::QLearning;
use rurel::strategy::terminate::FixedIterations;
use rurel::AgentTrainer;

/// Represents the possible actions a train agent can take in the simulation.
#[derive(PartialEq, Eq, Hash, Clone, Debug, Default)]
pub enum TrainAgentAction {
    /// Stop the train agent.
    #[default]
    Stop,
    /// Accelerate the train agent forward in milli meters per second squared (mm/s²).
    AccelerateForward {
        /// acceleration in milli meters per second squared (mm/s²).
        acceleration: i32,
    },
    /// Accelerate the train agent backward in milli meters per second squared (mm/s²).
    AccelerateBackward {
        /// acceleration in milli meters per second squared (mm/s²).
        acceleration: i32,
    },
}

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
    type A = TrainAgentAction;

    fn reward(&self) -> f64 {
        20.0 * self.speed_reward() + self.distance_reward()
    }

    fn actions(&self) -> Vec<TrainAgentAction> {
        let mut actions = vec![TrainAgentAction::Stop];
        for acceleration in 1..=(Self::MAX_ACCELERATION / Self::ACCELERATION_STEP) {
            actions.push(TrainAgentAction::AccelerateForward {
                acceleration: acceleration * Self::ACCELERATION_STEP,
            });
            actions.push(TrainAgentAction::AccelerateBackward {
                acceleration: acceleration * Self::ACCELERATION_STEP,
            });
        }
        actions
    }
}

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

    fn take_action(&mut self, action: &TrainAgentAction) {
        match action {
            TrainAgentAction::Stop => {
                self.state.current_speed_mm_s = 0;
            }
            TrainAgentAction::AccelerateForward { acceleration } => {
                self.state.current_speed_mm_s += acceleration * Self::TIME_DELTA_MS as i32 / 1000;
                self.state.delta_distance_mm =
                    self.state.current_speed_mm_s * Self::TIME_DELTA_MS as i32 / 1000;
            }
            TrainAgentAction::AccelerateBackward { acceleration } => {
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

/// A reinforcement learning agent that controls a train in the simulation.
#[derive(Default, Clone)]
pub struct TrainAgentAI {
    /// The railway graph representing the train network.
    pub railway_graph: Option<RailwayGraph>,
    /// The current node
    pub current_node: Option<i64>,
    /// The target node
    pub target_node: Option<i64>,
    /// The reinforcement learning agent responsible for controlling the train.
    pub agent_rl: TrainAgentRL,
    /// The trainer responsible for training the reinforcement learning agent.
    pub trainer: Arc<Mutex<AgentTrainer<TrainAgentState>>>,
}

impl fmt::Debug for TrainAgentAI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TrainAgentAI")
            .field("railway_graph", &self.railway_graph)
            .field("agent_rl", &self.agent_rl)
            // We use `Arc::as_ptr()` to display the pointer value of the `AgentTrainer` to avoid
            // issues with its non-Debug fields.
            .field("trainer", &format_args!("{:p}", Arc::as_ptr(&self.trainer)))
            .finish()
    }
}

impl TrainAgentAI {
    /// Creates a new `TrainAgentAI` with the given railway graph and initial state.
    ///
    /// # Arguments
    ///
    /// * `railway_graph` - The railway graph representing the train network.
    /// * `initial_state` - The initial state of the train agent in the simulation.
    ///
    /// # Returns
    ///
    /// A new `TrainAgentAI` instance.
    pub fn new(railway_graph: RailwayGraph, initial_state: TrainAgentState) -> Self {
        let agent_rl = TrainAgentRL {
            state: initial_state,
            max_speed_mm_s: ((160.0 / 3.6) as i32) * 1000,
        };
        let trainer = Arc::new(Mutex::new(AgentTrainer::new()));
        Self {
            railway_graph: Some(railway_graph),
            current_node: None,
            target_node: None,
            agent_rl,
            trainer,
        }
    }

    /// Trains the reinforcement learning agent for the specified number of iterations.
    ///
    /// # Arguments
    ///
    /// * `iterations` - The number of iterations to train the agent.
    pub fn train(&mut self, iterations: usize) {
        println!("Starting training for {} iterations...", iterations);
        let mut agent = self.agent_rl.clone();
        let mut trainer = self.trainer.try_lock().unwrap();
        trainer.train(
            &mut agent,
            &QLearning::new(0.2, 0.01, 20.),
            &mut FixedIterations::new(iterations as u32),
            &RandomExploration::new(),
        );
    }

    /// Returns the best action for the given state according to the trained reinforcement learning agent.
    ///
    /// # Arguments
    ///
    /// * `state` - The current state of the train agent in the simulation.
    ///
    /// # Returns
    ///
    /// The best action for the given state or `None` if no action can be selected.
    pub fn best_action(&self, state: &TrainAgentState) -> Option<TrainAgentAction> {
        Some(
            self.trainer
                .lock()
                .unwrap()
                .best_action(state)
                .unwrap_or_default(),
        )
    }

    /// Updates the current edge, target node (optionally), and calculates the new state by
    /// updating its distance using the shortest path distance while keeping the current speed constant.
    ///
    /// # Arguments
    ///
    /// * `current_edge` - The current edge on which the train agent is.
    /// * `target_node` - The optional target node the train agent is heading to.
    /// * `time_delta_ms` - The time delta in milliseconds since the last update.
    pub fn observe(
        &mut self,
        current_node: i64,
        target_node: Option<i64>,
        speed_mm_s: Option<i32>,
        delta_distance_mm: Option<i32>,
    ) {
        self.current_node = Some(current_node);
        self.target_node = target_node;

        let mut agent_state = self.agent_rl.state.clone();

        if let Some(speed) = speed_mm_s {
            agent_state.current_speed_mm_s = speed;
            agent_state.max_speed_percentage = 100 * speed / self.agent_rl.max_speed_mm_s;
        }

        if let Some(delta_distance_mm) = delta_distance_mm {
            agent_state.delta_distance_mm = delta_distance_mm;
        }
        self.agent_rl.state = agent_state;
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
            state: TrainAgentState {
                delta_distance_mm: 1000,
                current_speed_mm_s: 0,
                max_speed_percentage: 0,
            },
            max_speed_mm_s: (160.0 / (3.6 * 1000.0)) as i32,
        };

        // Test AccelerateForward action
        agent.take_action(&TrainAgentAction::AccelerateForward { acceleration: 1000 });
        assert_eq!(agent.state.current_speed_mm_s, 1000);
        assert_eq!(agent.state.delta_distance_mm, 1000);

        // Test AccelerateForward action
        agent.take_action(&TrainAgentAction::AccelerateForward { acceleration: 500 });
        assert_eq!(agent.state.current_speed_mm_s, 1500);
        assert_eq!(agent.state.delta_distance_mm, 1500);

        // Test AccelerateBackward action
        agent.take_action(&TrainAgentAction::AccelerateBackward { acceleration: 500 });
        assert_eq!(agent.state.current_speed_mm_s, 1000);
        assert_eq!(agent.state.delta_distance_mm, 1000);

        // Test Stop action
        agent.take_action(&TrainAgentAction::Stop);
        assert_eq!(agent.state.current_speed_mm_s, 0);
    }
}
