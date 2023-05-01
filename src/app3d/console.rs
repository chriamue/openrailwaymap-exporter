use crate::app3d::AppResource;
use crate::simulation::commands::{
    MetricsCommand, ObjectCommand, PauseCommand, SetSpeedupCommand, SimulationCommand,
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
    app.add_console_command::<MetricsCommand, _>(metrics_command);
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

impl Resource for MetricsCommand {}
impl NamedCommand for MetricsCommand {
    fn name() -> &'static str {
        "metrics"
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
    mut console_command: ConsoleCommand<SetSpeedupCommand>,
    mut app_resource: ResMut<AppResource>,
) {
    if let Some(Ok(command)) = console_command.take() {
        if let Some(simulation) = &app_resource.simulation.as_mut() {
            if let Ok(mut simulation) = simulation.lock() {
                if let Some(info) = command.execute(&mut simulation) {
                    console_command.reply_ok(info);
                }
            }
        }
    }
}

fn object_command(
    mut console_command: ConsoleCommand<ObjectCommand>,
    mut app_resource: ResMut<AppResource>,
) {
    if let Some(Ok(command)) = console_command.take() {
        if let Some(simulation) = &app_resource.simulation.as_mut() {
            if let Ok(mut simulation) = simulation.lock() {
                if let Some(info) = command.execute(&mut simulation) {
                    console_command.reply(info);
                }
            }
        }
    }
}

fn metrics_command(
    mut console_command: ConsoleCommand<MetricsCommand>,
    mut app_resource: ResMut<AppResource>,
) {
    if let Some(Ok(command)) = console_command.take() {
        if let Some(simulation) = &app_resource.simulation.as_mut() {
            if let Ok(mut simulation) = simulation.lock() {
                if let Some(info) = command.execute(&mut simulation) {
                    console_command.reply(info);
                }
            }
        }
    }
}
