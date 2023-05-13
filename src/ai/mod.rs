//! The `ai` module provides a reinforcement learning agent that controls a train in a railway network simulation.
//!
//! The module defines the `TrainAgentState` struct, which represents the state of a train agent in the simulation,
//! and the `TrainAgentAction` enum, which represents the possible actions a train agent can take in the simulation.
//!
//! The main component of this module is the `TrainAgentRL` struct, which implements the `rurel::mdp::Agent` trait
//! for the train agent. This struct handles the reinforcement learning process, updating the train agent's state
//! based on the actions taken and the rewards received.
//!
use std::any::Any;
use std::fmt;
use std::sync::{Arc, RwLock};

use crate::prelude::RailwayGraph;
use crate::simulation::agents::{DecisionAgent, RailMovableAction};
use crate::simulation::environment::ObservableEnvironment;
use crate::simulation::SimulationEnvironment;
use crate::types::RailwayObjectId;
use rurel::strategy::explore::RandomExploration;
use rurel::strategy::learn::QLearning;
use rurel::strategy::terminate::FixedIterations;
use rurel::AgentTrainer;
use uom::si::velocity::millimeter_per_second;

mod train_agent_state;
pub use train_agent_state::TrainAgentState;

mod train_agent_rl;
pub use train_agent_rl::TrainAgentRL;

/// A reinforcement learning agent that controls a train in the simulation.
#[derive(Default, Clone)]
pub struct TrainAgentAI {
    /// The id of the railway object
    pub id: RailwayObjectId,
    /// The railway graph representing the train network.
    pub railway_graph: Option<RailwayGraph>,
    /// The current node
    pub current_node: Option<i64>,
    /// The target node
    pub target_node: Option<i64>,
    /// The reinforcement learning agent responsible for controlling the train.
    pub agent_rl: TrainAgentRL,
    /// The trainer responsible for training the reinforcement learning agent.
    pub trainer: Arc<RwLock<AgentTrainer<TrainAgentState>>>,
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
        let trainer = Arc::new(RwLock::new(AgentTrainer::new()));
        Self {
            id: 0,
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
        let mut trainer = self.trainer.write().unwrap();
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
    pub fn best_action(&self, state: &TrainAgentState) -> Option<RailMovableAction> {
        Some(
            self.trainer
                .read()
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

impl DecisionAgent for TrainAgentAI {
    type A = RailMovableAction;

    fn next_action(&self, _delta_time: Option<std::time::Duration>) -> Self::A {
        self.best_action(&self.agent_rl.state).unwrap_or_default()
    }

    fn observe(&mut self, environment: &SimulationEnvironment) {
        if let Some(object) = environment.get_objects().iter().find(|o| o.id() == self.id) {
            self.current_node = object.position();
            self.target_node = object.next_target();
            let mut agent_state = self.agent_rl.state.clone();

            let speed = object.speed();

            agent_state.current_speed_mm_s = speed.get::<millimeter_per_second>() as i32;
            agent_state.max_speed_percentage = (100.0 * speed.get::<millimeter_per_second>()
                / self.agent_rl.max_speed_mm_s as f64)
                as i32;

            self.agent_rl.state = agent_state;
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_graph_vilbel;
    #[test]
    fn test_train_agent_ai() {
        let graph = test_graph_vilbel();

        let mut train_agent_ai = TrainAgentAI::new(graph, Default::default());

        let source_node = 662529467i64;
        let target_node = 662529466i64;

        train_agent_ai.observe(source_node, Some(target_node), Some(1000), Some(1000));
        train_agent_ai.train(10000);

        let state = TrainAgentState {
            delta_distance_mm: 1000,
            current_speed_mm_s: 1000,
            max_speed_percentage: 20,
        };

        let action = train_agent_ai.best_action(&state);
        assert_ne!(action, None);
    }
}
