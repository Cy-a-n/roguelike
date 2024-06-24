use std::{thread::sleep, time::Duration};

use bevy::{
    app::{App, Startup, Update},
    asset::Assets,
    diagnostic::{
        Diagnostic, DiagnosticsPlugin, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    pbr::{AmbientLight, PbrBundle, PointLight, PointLightBundle},
    prelude::{Camera3d, Camera3dBundle, Commands, Cuboid, IntoSystemConfigs, Res, ResMut, World},
    render::{camera::Camera, color::Color, mesh::Mesh},
    transform::components::Transform,
    window::{WindowMode, WindowPlugin},
    DefaultPlugins,
};
use bevy_spectator::{Spectator, SpectatorPlugin};
use material_handles::{MaterialHandles, MaterialHandlesPlugin};

use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Material editor".into(),
                    name: Some("material_viewer".into()),
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    mode: WindowMode::Fullscreen,
                    ..default()
                }),
                ..default()
            }),
            SpectatorPlugin,
            MaterialHandlesPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material_handles: Res<MaterialHandles>,
) {
    // Camera and window settings
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                clear_color: Color::BLACK.into(),
                hdr: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(2.0, 2.0, 2.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..Default::default()
        },
        Spectator,
    ));

    // Lighting
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 50.0,
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 50000000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(10.0, 10.0, 10.0),
        ..Default::default()
    });

    // Pbr bundles
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: material_handles.alice_blue().clone(), // The material to test.
        ..Default::default()
    });
}
