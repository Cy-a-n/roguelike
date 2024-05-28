use bevy::ecs::{
    component::Component,
    entity::Entity,
    query::With,
    system::{Commands, Query, Resource},
};

#[allow(clippy::needless_pass_by_value)]
pub fn cleanup<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_res<T: Resource>(mut commands: Commands) {
    commands.remove_resource::<T>();
}
