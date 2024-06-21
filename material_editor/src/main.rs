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
use iyes_perf_ui::{PerfUiCompleteBundle, PerfUiPlugin};
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
                    title: "I am a window!".into(),
                    name: Some("bevy.app".into()),
                    resolution: (500., 300.).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    mode: WindowMode::Fullscreen,
                    ..default()
                }),
                ..default()
            }),
            SpectatorPlugin,
            MaterialHandlesPlugin,
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            PerfUiPlugin,
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
            ..Default::default()
        },
        Spectator,
    ));

    commands.spawn(PerfUiCompleteBundle::default());

    // Lighting
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 10000.0,
    });
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 100000000.0,
    //         ..Default::default()
    //     },
    //     transform: Transform::from_xyz(10.0, 10.0, 10.0),
    //     ..Default::default()
    // });

    // Pbr bundles
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: material_handles.alice_blue().clone(), // The material to test.
        ..Default::default()
    });
}
