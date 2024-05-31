use bevy::{ecs::system::Resource, math::primitives::BoxedPolyline2d};

#[derive(Resource, Debug, Clone, PartialEq, Default)]
pub(super) struct CurrentLevel {
    // TODO!
    // The polylines used for ground collision, inverse kinematics and more.
    // I am using my own collision resolver, because I had trouble with [different bevy_dylib feature flags](https://github.com/rust-lang/cargo/issues/6313) when using bevy_rapier2d.
    // It might be more efficient to use a spatial indexing technique, but that would have to be unit tested.
    // walkable_polylines: Box<[BoxedPolyline2d]>,
    // climbable_polylines: Box<[BoxedPolyline2d]>,
    // impassable_polylines: Box<[BoxedPolyline2d]>,
}
