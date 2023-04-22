//! module with environment traits
use std::collections::HashMap;

use crate::{prelude::RailwayGraph, types::RailwayObjectId};

use super::SimulationObject;

/// A trait representing an environment that can be observed by decision agents.
///
/// Implementations of this trait should provide access to the railway graph and simulation objects.
pub trait ObservableEnvironment {
    /// Returns a reference to the railway graph.
    fn get_graph(&self) -> &RailwayGraph;

    /// Returns a vector of references to the simulation objects.
    fn get_objects(&self) -> Vec<&dyn SimulationObject>;
    /// This function takes a reference to self (which in this case is an ObservableEnvironment struct)
    /// and a reference to a RailwayObjectId object.
    fn get_object(&self, id: &RailwayObjectId) -> Option<&dyn SimulationObject>;
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

    fn get_object(&self, id: &RailwayObjectId) -> Option<&dyn SimulationObject> {
        self.objects.get(id).map(|boxed| &**boxed)
    }
}

impl ObservableEnvironmentRef for SimulationEnvironment {
    fn as_observable_env(&self) -> &(dyn ObservableEnvironment + 'static) {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_graph_vilbel;

    #[test]
    fn test_environment() {
        let graph = test_graph_vilbel();

        let environment = SimulationEnvironment {
            graph: graph.clone(),
            objects: HashMap::<i64, Box<dyn SimulationObject>>::default(),
        };

        assert_eq!(environment.graph, graph);
    }

    #[test]
    fn test_get_objects() {
        let graph = test_graph_vilbel();

        let mut environment = SimulationEnvironment {
            graph: graph.clone(),
            objects: HashMap::<i64, Box<dyn SimulationObject>>::default(),
        };
        let objects = environment.get_objects();
        assert_eq!(objects.len(), 0);

        let train = crate::railway_objects::Train::default();

        environment.objects.insert(0, Box::new(train));

        let objects = environment.get_objects();
        assert_eq!(objects.len(), 1);
    }
}
