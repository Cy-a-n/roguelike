use bevy::{
    app::{Plugin, Startup},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{component::Component, system::Commands},
    prelude::ClearColorConfig,
    render::{camera::Camera, color::Color},
};

pub(crate) struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, add_main_camera);
    }
}

fn add_main_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 0,
                hdr: false,
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            camera_2d: bevy::core_pipeline::core_2d::Camera2d,
            ..Default::default()
        },
        MainCamera,
    ));
}

#[derive(Component)]
struct MainCamera;
