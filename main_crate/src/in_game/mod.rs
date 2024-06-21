use bevy::{
    app::Plugin,
    ecs::schedule::{OnEnter, OnExit},
};
use bevy_utils::cleanup::cleanup;

use self::{components::CleanupInGame, systems::setup};

use super::GameStates;

mod bundles;
mod components;
mod resources;
mod systems;

pub(super) struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameStates::InGame), setup)
        .add_systems(OnExit(GameStates::InGame), cleanup::<CleanupInGame>)
            // .init_state::<InGameStates>()
            ;
    }
}

// #[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
// enum InGameStates {
//     #[default]

// }
