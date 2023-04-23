use crate::BddWorld;
use cucumber::{given, then, when};
use geo::{coord, HaversineDistance, Point};
use std::collections::VecDeque;
use uom::si::f64::Velocity;
use uom::si::velocity::kilometer_per_hour;

use openrailwaymap_exporter::railway_objects::GeoLocation;
use openrailwaymap_exporter::railway_objects::NextTarget;
use openrailwaymap_exporter::railway_objects::RailwayObject;
use openrailwaymap_exporter::railway_objects::Train;
use openrailwaymap_exporter::simulation::agents::ForwardUntilTargetAgent;
use openrailwaymap_exporter::simulation::Simulation;
use openrailwaymap_exporter::simulation::SimulationExecutor;

#[given(regex = "a train is placed at node (\\d+) with target (\\d+)")]
async fn given_train_placed_at_node_with_target(
    world: &mut BddWorld,
    start_node: i64,
    target_node: i64,
) {
    let mut simulation = Simulation::new(world.railway_graph.clone());
    let train = Train {
        id: 1,
        position: Some(start_node),
        geo_location: Some(coord! { x: 0.0, y: 0.0 }),
        next_target: Some(target_node),
        targets: VecDeque::from(vec![target_node]),
        max_speed: Velocity::new::<kilometer_per_hour>(80.0),
        ..Default::default()
    };
    let agent = ForwardUntilTargetAgent::new(train.id());
    simulation.add_object(Box::new(train.clone()), Some(Box::new(agent)));
    world.start_node = Some(start_node);
    world.simulation = Some(simulation);
}

#[given(regex = "a SimulationExecutor is created with (\\d+) fps and (\\d+) seconds")]
async fn given_simulation_executor_created(world: &mut BddWorld, fps: u32, run_time_secs: u64) {
    let simulation_executor = SimulationExecutor::new(fps, run_time_secs);
    world.simulation_executor = Some(simulation_executor);
}

#[when("the simulation is executed")]
async fn when_simulation_executed(world: &mut BddWorld) {
    if let (Some(ref mut simulation), Some(ref mut simulation_executor)) =
        (&mut world.simulation, &mut world.simulation_executor)
    {
        simulation_executor.execute(simulation);
    }
}

#[then("the train should be closer to the target node")]
async fn then_train_closer_to_target_node(world: &mut BddWorld) {
    let simulation = world.simulation.as_ref().unwrap();
    let graph = simulation.get_observable_environment().get_graph();
    let start_node = graph.get_node_by_id(world.start_node.unwrap()).unwrap();
    let train: &Train = simulation
        .get_observable_environment()
        .get_object(&1)
        .unwrap()
        .as_any()
        .downcast_ref::<Train>()
        .unwrap();
    let target_node = graph.get_node_by_id(train.next_target().unwrap()).unwrap();

    let start_location: Point<f64> = Point::new(start_node.lon, start_node.lat);
    let target_location: Point<f64> = Point::new(target_node.lon, target_node.lat);
    let current_location: Point<f64> = Point::new(
        train.geo_location().unwrap().x,
        train.geo_location().unwrap().y,
    );

    let distance_to_start = current_location.haversine_distance(&start_location);
    let distance_to_target = current_location.haversine_distance(&target_location);

    assert!(
        distance_to_target < distance_to_start,
        "Train is not closer to the target node."
    );
}
