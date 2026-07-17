use bevy::camera::visibility::RenderLayers;
use bevy::picking::prelude::MeshPickingCamera;
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_egui::{EguiGlobalSettings, PrimaryEguiContext};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub fn setup_camera(mut commands: Commands, mut egui_settings: ResMut<EguiGlobalSettings>) {
    egui_settings.auto_create_primary_context = false;

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 8_000.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera {
            radius: Some(8_000.0),
            button_orbit: MouseButton::Right,
            button_pan: MouseButton::Left,
            ..default()
        },
        MeshPickingCamera,
    ));

    commands.spawn((
        PrimaryEguiContext,
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        RenderLayers::none(),
    ));
}

pub struct CameraPlugins;

impl PluginGroup for CameraPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(PanOrbitCameraPlugin)
    }
}
