use bevy::{core_pipeline::core_2d::Camera2dBundle, ecs::world::World};

use super::components::MainCamera;

pub(super) fn setup(world: &mut World) {
    // Spawn main camera.
    world.spawn((Camera2dBundle::default(), MainCamera));
}
