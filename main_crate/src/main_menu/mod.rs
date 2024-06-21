use bevy::{
    app::{Plugin, Startup},
    ecs::schedule::{OnEnter, OnExit},
};
use bevy_utils::cleanup::cleanup;

use self::{components::CleanupMainMenu, systems::setup};

use super::GameStates;

mod bundles;
mod components;
mod resources;
mod systems;

pub(super) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameStates::MainMenu), setup)
        .add_systems(OnExit(GameStates::MainMenu), cleanup::<CleanupMainMenu>)
        // .init_state::<MainMenuStates>()
        ;
    }
}

// #[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
// enum MainMenuStates {
//     #[default]

// }
