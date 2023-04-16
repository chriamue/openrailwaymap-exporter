use std::collections::VecDeque;

use geo_types::coord;

use super::*;
use crate::railway_objects::{GeoLocation, Movable, MultipleTargets, NextTarget, RailwayObject};
use crate::simulation::agents::ForwardUntilTargetAgent;
use crate::{railway_objects::Train, tests::test_graph_vilbel};

#[test]
fn test_simulation_with_agent() {
    // Create a railway graph
    let graph = test_graph_vilbel();

    // Create a train
    let train = Train {
        id: 1,
        position: Some(0),
        geo_location: Some(coord! { x: 0.0, y: 0.0 }),
        next_target: Some(5),
        targets: VecDeque::from(vec![5, 10, 15]),
        ..Default::default()
    };

    // Create a simulation with the railway graph
    let mut simulation: Simulation<Train, ForwardUntilTargetAgent<Train>> = Simulation::new(graph);

    // Create an agent for the train
    let agent = ForwardUntilTargetAgent::new(train.clone());

    // Add the train and its agent to the simulation
    simulation.add_object(train, Some(agent));

    // Update the simulation with a given delta_time
    let delta_time = Duration::from_secs(1);
    simulation.update(delta_time);

    // Get the updated train object from the simulation
    let updated_train = simulation.objects.get(&1).unwrap();

    // Test the expected outcome, e.g., check if the train's speed has increased
    assert_eq!(updated_train.speed(), 20.0); // Assuming the initial speed was 0
}
