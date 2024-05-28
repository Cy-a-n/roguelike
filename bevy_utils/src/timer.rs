extern crate proc_macro;
use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    time::{Time, Timer, TimerMode},
};

// A component or resource with a single timer.
pub trait HasTimer {
    fn timer(&mut self) -> &mut Timer;
}

/// Ticks the timer and removes it if it is `TimerMode::Once` and finished.
pub fn run_timer<T: HasTimer + Component>(
    mut commands: Commands,
    mut timer: Query<(Entity, &mut T)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in &mut timer {
        let timer = timer.timer();

        timer.tick(time.delta());
        if timer.finished() && timer.mode() == TimerMode::Once {
            commands.entity(entity).despawn();
        }
    }
}

/// Ticks the timer and removes it if it is `TimerMode::Once` and finished.
pub fn run_timer_res<T: HasTimer + Resource>(
    mut commands: Commands,
    mut timer: ResMut<T>,
    time: Res<Time>,
) {
    let timer = timer.timer();

    timer.tick(time.delta());
    if timer.finished() && timer.mode() == TimerMode::Once {
        commands.remove_resource::<T>();
    }
}
