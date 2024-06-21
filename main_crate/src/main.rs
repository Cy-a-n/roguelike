use bevy::{app::App, DefaultPlugins};
use main_crate::GameStatesPlugin;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GameStatesPlugin))
        .run();
}
