use bevy::{app::App, DefaultPlugins};
use main_camera::MainCameraPlugin;

mod main_camera;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MainCameraPlugin))
        .run();
}
