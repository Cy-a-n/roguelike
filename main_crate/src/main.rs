use bevy::{app::App, DefaultPlugins};
use game_state::GameStatesPlugin;

mod game_state;
/// Reusable mathematical utilities.
pub mod math;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GameStatesPlugin))
        .run();
}
