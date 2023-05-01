use crate::simulation::commands::SimulationCommand;
use crate::simulation::Simulation;
use clap::Parser;

/// A command for querying object information in a simulation.
///
/// This command has two subcommands:
///
/// * `list` - Lists the IDs of all objects in the simulation.
/// * `show` - Shows detailed information about an object with a specified ID.
///
/// Example usage:
///
/// ```ignore
/// // List all objects
/// let list_cmd = ObjectCommand::List;
/// let list_output = list_cmd.execute(&mut simulation);
///
/// // Show information about an object with ID 1
/// let show_cmd = ObjectCommand::Show(ShowObject { object_id: 1 });
/// let show_output = show_cmd.execute(&mut simulation);
/// ```
#[derive(Parser, Debug)]
#[command(name = "object")]
pub enum ObjectCommand {
    /// Lists the IDs of all objects in the simulation.
    #[command(name = "list")]
    List,

    /// Shows detailed information about an object with a specified ID.
    #[command(name = "show")]
    Show(ShowObject),
}

/// A command for showing information about an object with a specified ID.
#[derive(Parser, Debug)]
#[command(name = "show")]
pub struct ShowObject {
    /// The ID of the object to show information about.
    #[clap(name = "ID")]
    pub object_id: i64,
}

impl SimulationCommand for ObjectCommand {
    fn execute(&self, simulation: &mut Simulation) -> Option<String> {
        match self {
            ObjectCommand::List => {
                let object_ids: Vec<i64> = simulation.environment.objects.keys().copied().collect();
                Some(format!("Objects: {:?}", object_ids))
            }
            ObjectCommand::Show(show_object) => {
                if let Some(object) = simulation.environment.objects.get(&show_object.object_id) {
                    Some(format!("Object {}: {:?}", show_object.object_id, object))
                } else {
                    Some(format!("Object {} not found", show_object.object_id))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::railway_objects::Train;

    #[test]
    fn test_object_command() {
        let mut simulation = Simulation::new(Default::default());

        // Add an object to the simulation
        let object = Train {
            id: 1,
            ..Default::default()
        };

        simulation.environment.objects.insert(1, Box::new(object));

        // Test the `list` subcommand
        let list_command = ObjectCommand::List;
        let list_output = list_command.execute(&mut simulation);
        assert_eq!(list_output, Some("Objects: [1]".to_string()));

        // Test the `show` subcommand
        let show_command = ObjectCommand::Show(ShowObject { object_id: 1 });
        let show_output = show_command.execute(&mut simulation);
        assert!(show_output.unwrap().starts_with("Object 1: Train { id: 1"));

        // Test the `show` subcommand with a non-existent object ID
        let show_command = ObjectCommand::Show(ShowObject { object_id: 2 });
        let show_output = show_command.execute(&mut simulation);
        assert_eq!(show_output, Some("Object 2 not found".to_string()));
    }
}
