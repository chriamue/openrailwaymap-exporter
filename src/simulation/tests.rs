use super::*;
use crate::railway_algorithms::RailwayEdgeAlgos;
use crate::railway_objects::Train;
use crate::simulation::agents::ForwardUntilTargetAgent;
use crate::simulation::commands::{SetSpeedupCommand, SimulationCommand};
use crate::tests::test_graph_1;
use approx::assert_relative_eq;
use geo::coord;
use std::collections::VecDeque;
use uom::si::{
    f64::Velocity,
    velocity::{kilometer_per_hour, meter_per_second},
};

#[test]
fn test_debug_implementation() {
    let graph = test_graph_1();

    let mut simulation = Simulation::new(graph);

    let train = Train {
        id: 1,
        position: Some(1),
        geo_location: Some(coord! { x: 0.0, y: 0.0 }),
        next_target: Some(2),
        targets: VecDeque::from(vec![2, 10, 15]),
        max_speed: Velocity::new::<kilometer_per_hour>(80.0),
        ..Default::default()
    };
    simulation.add_object(Box::new(train), None);

    let debug_output = format!("{:?}", simulation);

    assert!(debug_output.contains("Simulation"));
    assert!(debug_output.contains("railway_objects: 1"));
}

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
        max_speed: Velocity::new::<kilometer_per_hour>(80.0),
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
    assert_eq!(
        updated_train.speed(),
        Velocity::new::<meter_per_second>(20.0)
    ); // Assuming the initial speed was 0

    let current_location = train.geo_location().unwrap();
    let current_speed = Velocity::new::<meter_per_second>(20.0);
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
    let distance_to_travel = current_speed * Time::new::<second>(delta_time.as_secs_f64());
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

#[test]
fn test_get_observable_environment() {
    let graph = test_graph_1();
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

    let train = simulation.get_observable_environment().get_object(&1);
    assert!(train.is_some());
    let train = train.unwrap().as_any().downcast_ref::<Train>();

    assert!(train.is_some());
    assert_eq!(1, train.unwrap().id());
}

#[test]
fn test_metrics_count_stop_actions() {
    let graph = test_graph_1();

    let mut simulation = Simulation::new(graph);

    let train = Train {
        id: 1,
        position: Some(1),
        geo_location: Some(coord! { x: 0.0, y: 0.0 }),
        next_target: Some(2),
        targets: VecDeque::from(vec![2, 10, 15]),
        max_speed: Velocity::new::<kilometer_per_hour>(80.0),
        ..Default::default()
    };
    let agent = ForwardUntilTargetAgent::new(train.id());
    simulation.add_object(Box::new(train), Some(Box::new(agent)));

    let action_count_handler = ActionCountHandler::new();
    simulation.register_metrics_handler(Box::new(action_count_handler));

    let total_steps = 10;

    for _ in 0..total_steps {
        simulation.update(Duration::from_secs(1));
    }

    let stop_count = simulation.metrics_handlers.get(0).unwrap().get_value();

    let expected_stop_count = 10.0;
    assert_eq!(stop_count, expected_stop_count);
}

#[test]
fn test_simulation_with_agent_and_speedup() {
    let graph = test_graph_1();
    let train = Train {
        id: 1,
        position: Some(1),
        geo_location: Some(coord! { x: 0.0, y: 0.0 }),
        next_target: Some(2),
        targets: VecDeque::from(vec![2, 10, 15]),
        max_speed: Velocity::new::<kilometer_per_hour>(80.0),
        ..Default::default()
    };
    let mut simulation: Simulation = Simulation::new(graph);
    let agent = ForwardUntilTargetAgent::new(train.id());
    simulation.add_object(Box::new(train.clone()), Some(Box::new(agent)));

    // Set the speedup factor to 2.0
    let set_speedup_command = SetSpeedupCommand {
        speedup_factor: 2.0,
    };
    set_speedup_command.execute(&mut simulation);

    // Update the simulation with a given delta_time
    let delta_time = Duration::from_secs(1);
    simulation.update(delta_time);
    let updated_train = simulation.environment.objects.get(&1).unwrap();

    // Test the expected outcome, e.g., check if the train's speed has increased
    assert_relative_eq!(
        updated_train.speed().get::<meter_per_second>(),
        22.22,
        epsilon = 0.1
    );
}
