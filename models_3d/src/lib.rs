use bevy::app::Plugin;
use material_handles::MaterialHandlesPlugin;

mod bundles;
mod components;
mod resources;
mod systems;

pub use material_handles::MaterialHandles;

pub struct Models3DPlugin;

impl Plugin for Models3DPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(MaterialHandlesPlugin);
        todo!()
    }
}
