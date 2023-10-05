use crate::{
    collision::{Collider, ColliderShape, RigidBody},
    player::*,
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct HeroBundle {
    pub name: Name,
    pub marker: PlayerMarker,
    pub motor: PlayerMotor,
    pub computed_visibility: ComputedVisibility,

    pub visibility: Visibility,
    pub collider: Collider,
    pub rb: RigidBody,

    #[bundle()]
    pub transform: TransformBundle,
}

impl Default for HeroBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Hero"),
            visibility: Visibility::Visible,
            marker: Default::default(),
            motor: Default::default(),
            computed_visibility: Default::default(),
            transform: Default::default(),
            collider: Collider {
                dynamic: true,
                shape: ColliderShape::circle(4.),
            },
            rb: RigidBody::new_resting(1.),
        }
    }
}
