use crate::player::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct HeroBundle {
    pub name: Name,
    pub marker: PlayerMarker,
    pub motor: PlayerMotor,

    #[bundle()]
    pub visibility: VisibilityBundle,

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
            visibility: Default::default(),
            marker: Default::default(),
            motor: Default::default(),
            transform: Default::default(),
            rb: RigidBody::Dynamic,
            collider: Collider::ball(3.5),
            velocity_controller: Velocity::default(),
            sleeping: Sleeping::disabled(),
            axis_lock: LockedAxes::ROTATION_LOCKED_Z,
        }
    }
}
