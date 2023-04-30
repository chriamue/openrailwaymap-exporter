use crate::app3d::AppResource;
use crate::simulation::commands::{PauseCommand, SimulationCommand};
use bevy::prelude::*;
use bevy_console::{
    AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsolePlugin, NamedCommand,
};

pub fn add_console_to_app(app: &mut App) {
    app.add_plugin(ConsolePlugin);
    app.add_console_command::<PauseCommand, _>(pause_command);
    app.insert_resource(ConsoleConfiguration {
        ..Default::default()
    });
}

impl Resource for PauseCommand {}
impl NamedCommand for PauseCommand {
    fn name() -> &'static str {
        "pause"
    }
}

fn pause_command(mut app_resource: ResMut<AppResource>, mut pause: ConsoleCommand<PauseCommand>) {
    if let Some(Ok(pause_command)) = pause.take() {
        if let Some(simulation) = &app_resource.simulation.as_mut() {
            if let Ok(mut simulation) = simulation.lock() {
                if let Some(info) = pause_command.execute(&mut simulation) {
                    pause.reply_ok(info);
                }
            }
        }
    }
}
