//! This module provides a set of commands that can be used to manipulate a `Simulation` instance.
//! Each command implements the `SimulationCommand` trait, which provides the `execute` method to
//! apply the command on a given simulation instance.
//!
use crate::simulation::Simulation;
use clap::Parser;

/// A trait for commands that manipulate a simulation.
pub trait SimulationCommand {
    /// Executes the command on the given simulation instance.
    ///
    /// # Arguments
    ///
    /// * `simulation` - A mutable reference to the `Simulation` instance.
    ///
    /// # Returns
    ///
    /// * An optional `String` message describing the action taken by the command.
    fn execute(&self, simulation: &mut Simulation) -> Option<String>;
}

/// A command that toggles the pause state of a simulation.
#[derive(Parser, Debug)]
#[command(name = "pause")]
pub struct PauseCommand {}

impl SimulationCommand for PauseCommand {
    fn execute(&self, simulation: &mut Simulation) -> Option<String> {
        simulation.is_paused = !simulation.is_paused;
        let msg = if simulation.is_paused {
            "Simulation paused"
        } else {
            "Simulation resumed"
        };
        Some(msg.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pause_command() {
        let mut simulation = Simulation::new(Default::default());
        let pause_command = PauseCommand {};

        // Initially, the simulation should be unpaused.
        assert_eq!(simulation.is_paused, false);

        // Test pausing the simulation.
        let result = pause_command.execute(&mut simulation);
        assert_eq!(simulation.is_paused, true);
        assert_eq!(result, Some("Simulation paused".to_string()));

        // Test unpausing the simulation.
        let result = pause_command.execute(&mut simulation);
        assert_eq!(simulation.is_paused, false);
        assert_eq!(result, Some("Simulation resumed".to_string()));
    }
}
