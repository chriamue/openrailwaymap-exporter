use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_mod_picking::PickingCameraBundle;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        PanOrbitCamera {
            radius: 1000.0,
            button_orbit: MouseButton::Right,
            button_pan: MouseButton::Left,
            ..default()
        },
        PickingCameraBundle::default(),
    ));
}

pub struct CameraPlugins;

impl PluginGroup for CameraPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(PanOrbitCameraPlugin)
    }
}
