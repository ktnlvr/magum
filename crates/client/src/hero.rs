use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Component, PartialEq, Hash, Reflect)]
pub struct PlayerMarker;

pub type IsPlayer = With<PlayerMarker>;

#[derive(Component)]
pub struct CameraDirector {
    pub desired_location: Vec3,
    pub follow_speed: f32,
}

impl Default for CameraDirector {
    fn default() -> Self {
        Self {
            desired_location: Vec3::ZERO,
            follow_speed: 30.,
        }
    }
}

#[derive(Bundle)]
pub struct CameraBundle {
    pub director: CameraDirector,
    pub camera: Camera2dBundle,
}

impl Default for CameraBundle {
    fn default() -> Self {
        Self {
            director: CameraDirector::default(),

            camera: Camera2dBundle {
                projection: OrthographicProjection {
                    far: 1000.,
                    near: -1000.,
                    scale: 0.2,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct PlayerMotor {
    pub wish_direction: Vec2,
    pub velocity: Vec2,

    pub drag: f32,
    pub max_speed: f32,
    pub max_accel: f32,
}

impl Default for PlayerMotor {
    fn default() -> Self {
        Self {
            wish_direction: Vec2::ZERO,
            velocity: Vec2::ZERO,
            drag: 9.,
            max_speed: 3.,
            max_accel: 4.,
        }
    }
}

#[derive(Default, Bundle)]
pub struct HeroBundle {
    pub marker: PlayerMarker,
    pub motor: PlayerMotor,

    #[bundle()]
    pub sprite: SpriteSheetBundle,
}
