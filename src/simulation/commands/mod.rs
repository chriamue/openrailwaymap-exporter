//! This module provides a set of commands that can be used to manipulate a `Simulation` instance.
//! Each command implements the `SimulationCommand` trait, which provides the `execute` method to
//! apply the command on a given simulation instance.
//!
use clap::{arg, Parser};
mod object_command;
use crate::simulation::Simulation;
pub use object_command::ObjectCommand;

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

/// A command that sets the speedup factor of the simulation.
#[derive(Parser, Debug)]
#[command(name = "speedup")]
pub struct SetSpeedupCommand {
    /// The speedup factor to set
    #[arg(default_value = "1.0")]
    pub speedup_factor: f64,
}

impl SimulationCommand for SetSpeedupCommand {
    fn execute(&self, simulation: &mut Simulation) -> Option<String> {
        simulation.speedup_factor = self.speedup_factor;
        Some(format!("Speedup factor set to {}", self.speedup_factor))
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

    #[test]
    fn test_set_speedup_command() {
        let mut simulation = Simulation::new(Default::default());
        let set_speedup_command_2 = SetSpeedupCommand {
            speedup_factor: 2.0,
        };

        // Initially, the speedup factor should be 1.0 (default value).
        assert_eq!(simulation.speedup_factor, 1.0);

        // Test setting the speedup factor to 2.0.
        let result = set_speedup_command_2.execute(&mut simulation);
        assert_eq!(simulation.speedup_factor, 2.0);
        assert_eq!(result, Some("Speedup factor set to 2".to_string()));

        // Test setting the speedup factor to the default value (1.0).
        let set_speedup_command_default = SetSpeedupCommand {
            speedup_factor: 1.0,
        };
        let result = set_speedup_command_default.execute(&mut simulation);
        assert_eq!(simulation.speedup_factor, 1.0);
        assert_eq!(result, Some("Speedup factor set to 1".to_string()));
    }
}
