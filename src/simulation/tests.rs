use super::*;
use crate::railway_objects::Train;
use crate::simulation::agents::ForwardUntilTargetAgent;
use crate::tests::test_graph_1;
use geo::coord;
use std::collections::VecDeque;

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
    simulation.add_object(Box::new(train.clone()), Some(Box::new(agent)));

    // Update the simulation with a given delta_time
    let delta_time = Duration::from_secs(1);
    simulation.update(delta_time);

    // Get the updated train object from the simulation
    let updated_train = simulation.environment.objects.get(&1).unwrap();

    // Test the expected outcome, e.g., check if the train's speed has increased
    assert_eq!(updated_train.speed(), 20.0); // Assuming the initial speed was 0

    let current_location = train.geo_location().unwrap();
    let current_speed = 20.0;
    let next_node_id = simulation
        .environment
        .graph
        .get_next_node(train.position().unwrap(), train.next_target().unwrap())
        .unwrap();
    let direction_node = &simulation.environment.graph.graph[*simulation
        .environment
        .graph
        .node_indices
        .get(&next_node_id)
        .unwrap()];
    let direction_coord = coord! { x: direction_node.lon, y: direction_node.lat };
    let distance_to_travel = current_speed * delta_time.as_secs_f64();
    let edge = simulation
        .environment
        .graph
        .railway_edge(train.position().unwrap(), next_node_id)
        .expect("Invalid edge");
    let expected_new_location =
        edge.position_on_edge(current_location, distance_to_travel, direction_coord);

    // Check if the train's geo_location has changed after the update
    assert_ne!(
        updated_train.geo_location().unwrap(),
        train.geo_location().unwrap()
    );

    // Test if the updated train's new geo_location is as expected
    assert_eq!(updated_train.geo_location().unwrap(), expected_new_location);
}
