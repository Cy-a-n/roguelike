use bevy::ecs::component::Component;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(super) struct CleanupInGame;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(super) struct CurrentLevel {}
