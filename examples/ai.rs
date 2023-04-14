// examples/ai.rs

use openrailwaymap_exporter::ai::{TrainAgentAI, TrainAgentState};
use openrailwaymap_exporter::importer::overpass_importer::{from_railway_elements, RailwayElement};
use rurel::mdp::Agent;

fn railway_elements() -> Vec<RailwayElement> {
    let test_data = serde_json::from_slice(include_bytes!("../src/tests/res/vilbel.json"))
        .expect("Failed to deserialize the JSON data");
    RailwayElement::from_json(&test_data).unwrap()
}

fn simulate(agent: &mut TrainAgentAI, initial_state: &TrainAgentState) {
    println!(
        "Starting simulation with initial state: {:?}",
        initial_state
    );
    agent.agent_rl.state = initial_state.clone();
    let mut step = 0;
    while step < 100 {
        let best_action = agent.best_action(agent.agent_rl.current_state()).unwrap();
        agent.agent_rl.take_action(&best_action);
        println!(
            "Step {}: Took action: {:?}, New state: {:?}",
            step,
            best_action,
            agent.agent_rl.current_state()
        );
        step += 1;
    }
}

fn main() {
    let railway_elements = railway_elements();
    let graph = from_railway_elements(&railway_elements);

    let initial_state = TrainAgentState {
        delta_distance_mm: 0,
        current_speed_mm_s: 0,
        max_speed_percentage: 0,
    };

    let mut agent = TrainAgentAI::new(graph, initial_state.clone());
    agent.train(10000);

    simulate(&mut agent, &initial_state);
}
