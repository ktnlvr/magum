use crate::player::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct HeroBundle {
    pub name: Name,
    pub marker: PlayerMarker,
    pub motor: PlayerMotor,
    
    pub computed_visibility: ComputedVisibility,
    pub visibility: Visibility,

    pub rb: RigidBody,
    pub collider: Collider,
    pub velocity_controller: Velocity,
    pub sleeping: Sleeping,
    pub axis_lock: LockedAxes,

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
            rb: RigidBody::Dynamic,
            collider: Collider::ball(3.5),
            velocity_controller: Velocity::default(),
            sleeping: Sleeping::disabled(),
            axis_lock: LockedAxes::ROTATION_LOCKED_Z,
        }
    }
}
