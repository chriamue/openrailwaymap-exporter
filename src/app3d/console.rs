use crate::app3d::AppResource;
use crate::simulation::commands::{
    ObjectCommand, PauseCommand, SetSpeedupCommand, SimulationCommand,
};
use bevy::prelude::*;
use bevy_console::{
    AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsolePlugin, NamedCommand,
};

pub fn add_console_to_app(app: &mut App) {
    app.add_plugin(ConsolePlugin);
    app.add_console_command::<PauseCommand, _>(pause_command);
    app.add_console_command::<SetSpeedupCommand, _>(set_speedup_command);
    app.add_console_command::<ObjectCommand, _>(object_command);
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

impl Resource for SetSpeedupCommand {}
impl NamedCommand for SetSpeedupCommand {
    fn name() -> &'static str {
        "speedup"
    }
}

impl Resource for ObjectCommand {}
impl NamedCommand for ObjectCommand {
    fn name() -> &'static str {
        "object"
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

fn set_speedup_command(
    mut command: ConsoleCommand<SetSpeedupCommand>,
    mut app_resource: ResMut<AppResource>,
) {
    if let Some(Ok(pause_command)) = command.take() {
        if let Some(simulation) = &app_resource.simulation.as_mut() {
            if let Ok(mut simulation) = simulation.lock() {
                if let Some(info) = pause_command.execute(&mut simulation) {
                    command.reply_ok(info);
                }
            }
        }
    }
}

fn object_command(
    mut command: ConsoleCommand<ObjectCommand>,
    mut app_resource: ResMut<AppResource>,
) {
    if let Some(Ok(object_command)) = command.take() {
        if let Some(simulation) = &app_resource.simulation.as_mut() {
            if let Ok(mut simulation) = simulation.lock() {
                if let Some(info) = object_command.execute(&mut simulation) {
                    command.reply(info);
                }
            }
        }
    }
}
