use std::collections::VecDeque;

use geo_types::coord;

use super::*;
use crate::railway_objects::Train;
use crate::simulation::agents::ForwardUntilTargetAgent;
use crate::tests::test_graph_1;

#[test]
fn test_simulation_with_agent() {
    // Create a railway graph
    let graph = test_graph_1();

    // Create a train
    let train = Train {
        id: 1,
        position: Some(1),
        geo_location: Some(coord! { x: 0.0, y: 0.0 }),
        next_target: Some(2),
        targets: VecDeque::from(vec![2, 10, 15]),
        ..Default::default()
    };

    // Create a simulation with the railway graph
    let mut simulation: Simulation = Simulation::new(graph);

    // Create an agent for the train
    let agent = ForwardUntilTargetAgent::new(train.id());

    // Add the train and its agent to the simulation
    simulation.add_object(Box::new(train), Some(Box::new(agent)));

    // Update the simulation with a given delta_time
    let delta_time = Duration::from_secs(1);
    simulation.update(delta_time);

    // Get the updated train object from the simulation
    let updated_train = simulation.environment.objects.get(&1).unwrap();

    // Test the expected outcome, e.g., check if the train's speed has increased
    assert_eq!(updated_train.speed(), 20.0); // Assuming the initial speed was 0

    let expected_new_position = 1; // Calculate the expected new position based on the train's speed and delta_time
    assert_eq!(updated_train.position().unwrap(), expected_new_position);
}
