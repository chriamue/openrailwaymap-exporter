// examples/ai.rs

use openrailwaymap_exporter::ai::TrainAgentState;
use openrailwaymap_exporter::importer::overpass_importer::{from_railway_elements, RailwayElement};
use rurel::mdp::Agent;
use rurel::strategy::explore::RandomExploration;
use rurel::strategy::learn::QLearning;
use rurel::strategy::terminate::FixedIterations;
use rurel::AgentTrainer;

use openrailwaymap_exporter::prelude::RailwayGraph;

fn railway_elements() -> Vec<RailwayElement> {
    let test_data = serde_json::from_slice(include_bytes!("../src/tests/res/vilbel.json"))
        .expect("Failed to deserialize the JSON data");
    RailwayElement::from_json(&test_data).unwrap()
}

fn simulate(
    trainer: &AgentTrainer<TrainAgentState>,
    initial_state: &TrainAgentState,
    railway_graph: &RailwayGraph,
) {
    let mut agent = openrailwaymap_exporter::ai::TrainAgentRL {
        railway_graph: Some(railway_graph.clone()),
        state: initial_state.clone(),
    };

    println!(
        "Starting simulation with initial state: {:?}",
        initial_state
    );
    let mut step = 0;
    while step < 100 {
        let best_action = trainer.best_action(agent.current_state()).unwrap();
        agent.take_action(&best_action);
        println!(
            "Step {}: Took action: {:?}, New state: {:?}",
            step,
            best_action,
            agent.current_state()
        );
        step += 1;
    }
}

fn main() {
    let railway_elements = railway_elements();
    let graph = from_railway_elements(&railway_elements);

    let initial_state = TrainAgentState {
        remaining_distance_mm: 1000 * 1000,
        current_speed_mm_s: 0,
        max_speed_mm_s: ((160.0 / 3.6) as i32) * 1000,
        time_delta_ms: 1000,
    };

    let mut trainer = AgentTrainer::new();
    let mut agent = openrailwaymap_exporter::ai::TrainAgentRL {
        railway_graph: Some(graph.clone()),
        state: initial_state.clone(),
    };

    trainer.train(
        &mut agent,
        &QLearning::new(0.2, 0.01, 2.),
        &mut FixedIterations::new(100000),
        &RandomExploration::new(),
    );

    simulate(&trainer, &initial_state, &graph);
}
