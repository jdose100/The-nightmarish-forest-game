//! This module contains various components that
//! can't be placed in a separate module for various reasons.
use bevy::prelude::*;

/// Sphere of tear - this is the component that determines
/// in what radius the player receives fear.
#[derive(Component, Clone, Copy, Reflect)]
#[reflect(Component)] #[require(Transform)]
pub struct SphereOfTear(pub f32);

impl SphereOfTear {
    /// Return true if point in sphere, else false
    pub fn point_in_sphere(&self, sphere_position: &Vec3, point: &Vec3) -> bool {
        let value: f32 =
            (sphere_position.x - point.x).powf(2.0) +
            (sphere_position.y - point.y).powf(2.0) +
            (sphere_position.z - point.z).powf(2.0);

        value <= self.0
    }
}

