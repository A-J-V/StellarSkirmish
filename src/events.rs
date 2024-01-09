use crate::components::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub damage_value: f32,
    pub damage_type: DamageType,
}

#[derive(Event)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub new_velocity: Vec3,
}

// Can't have optional fields in Rust. I would like to be able to have optional marker components.
// Is there a better pattern for that? Need a generic bag of components that I can dynamically spawn
// during runtime.
#[derive(Event)]
pub struct SpawnGuidedMissileEvent {
    pub transform: Transform,
}
