//! This module contains a factory for creating decision agents.
//!
//! The factory provides a list of options for different decision agent implementations,
//! such as TrainAgentAI and ForwardUntilTargetAgent.

use super::DecisionAgent;
use super::ForwardUntilTargetAgent;
use super::RailMovableAction;
#[cfg(feature = "ai")]
use crate::ai::{TrainAgentAI, TrainAgentState};
use crate::simulation::environment::ObservableEnvironment;
use crate::simulation::SimulationEnvironment;
use crate::types::RailwayObjectId;

/// An enumeration of available decision agent implementations.
#[derive(Default, Debug, PartialEq)]
pub enum DecisionAgentOption {
    /// TrainAgentAI implementation, requires the "ai" feature to be enabled.
    #[cfg(feature = "ai")]
    TrainAgentAI,

    /// ForwardUntilTargetAgent implementation.
    #[default]
    ForwardUntilTargetAgent,
}

/// A factory for creating decision agents.
pub struct DecisionAgentFactory;

impl DecisionAgentFactory {
    /// Create a decision agent based on the provided option.
    ///
    /// # Arguments
    ///
    /// * `option` - The selected decision agent implementation.
    /// * `id` - The RailwayObjectId for the agent.
    /// * `environment` - A reference to a SimulationEnvironment.
    ///
    /// # Returns
    ///
    /// * A boxed `DecisionAgent` trait object with the specified implementation.
    pub fn create_decision_agent(
        option: DecisionAgentOption,
        id: RailwayObjectId,
        environment: &SimulationEnvironment,
    ) -> Box<dyn DecisionAgent<A = RailMovableAction>> {
        match option {
            #[cfg(feature = "ai")]
            DecisionAgentOption::TrainAgentAI => {
                let mut train_agent_ai =
                    TrainAgentAI::new(environment.get_graph().clone(), TrainAgentState::default());
                train_agent_ai.train(10000);
                Box::new(train_agent_ai)
            }
            DecisionAgentOption::ForwardUntilTargetAgent => {
                Box::new(ForwardUntilTargetAgent::new(id))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::railway_model::RailwayGraph;
    use std::collections::HashMap;

    #[test]
    fn test_create_decision_agent() {
        let environment = SimulationEnvironment {
            graph: RailwayGraph::default(),
            objects: HashMap::new(),
        };
        let id = 1;

        // Test creation of ForwardUntilTargetAgent
        let agent = DecisionAgentFactory::create_decision_agent(
            DecisionAgentOption::ForwardUntilTargetAgent,
            id,
            &environment,
        );
        assert!(agent.as_any().is::<ForwardUntilTargetAgent>());

        // Test creation of TrainAgentAI if the "ai" feature is enabled
        #[cfg(feature = "ai")]
        {
            let agent = DecisionAgentFactory::create_decision_agent(
                DecisionAgentOption::TrainAgentAI,
                id,
                &environment,
            );
            assert!(agent.as_any().is::<TrainAgentAI>());
        }
    }
}
