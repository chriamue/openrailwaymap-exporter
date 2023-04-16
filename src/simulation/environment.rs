//! module with environment traits
use std::collections::HashMap;

use crate::prelude::RailwayGraph;

use super::SimulationObject;

/// A trait representing an environment that can be observed by decision agents.
///
/// Implementations of this trait should provide access to the railway graph and simulation objects.
pub trait ObservableEnvironment {
    /// Returns a reference to the railway graph.
    fn get_graph(&self) -> &RailwayGraph;

    /// Returns a vector of references to the simulation objects.
    fn get_objects(&self) -> Vec<&dyn SimulationObject>;
}

/// A trait representing a reference to an observable environment.
///
/// Implementations of this trait should provide a method to obtain a reference to an `ObservableEnvironment` object.
pub trait ObservableEnvironmentRef {
    /// Returns a reference to an `ObservableEnvironment` object.
    fn as_observable_env(&self) -> &(dyn ObservableEnvironment + 'static);
}

/// Represents the environment for the decision agent, containing the `RailwayGraph`.
#[derive(Debug)]
pub struct SimulationEnvironment {
    /// A reference to the `RailwayGraph` in the environment.
    pub graph: RailwayGraph,
    /// A collection of simulation objects, keyed by their unique identifiers.
    pub objects: HashMap<i64, Box<dyn SimulationObject>>,
}

impl ObservableEnvironment for SimulationEnvironment {
    fn get_graph(&self) -> &RailwayGraph {
        &self.graph
    }

    fn get_objects(&self) -> Vec<&dyn SimulationObject> {
        self.objects
            .values()
            .map(|object| object.as_ref())
            .collect()
    }
}

impl ObservableEnvironmentRef for SimulationEnvironment {
    fn as_observable_env(&self) -> &(dyn ObservableEnvironment + 'static) {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::test_graph_vilbel;

    use super::*;

    #[test]
    fn test_environment() {
        let graph = test_graph_vilbel();

        let environment = SimulationEnvironment {
            graph: graph.clone(),
            objects: HashMap::<i64, Box<dyn SimulationObject>>::default(),
        };

        assert_eq!(environment.graph, graph);
    }
}
