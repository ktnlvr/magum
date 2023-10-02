use bevy::{prelude::*, window::PrimaryWindow};

use crate::player::PlayerMarker;

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

pub fn move_camera(
    mut camera: Query<
        (
            &mut Transform,
            &GlobalTransform,
            &Camera,
            &mut CameraDirector,
        ),
        Without<PlayerMarker>,
    >,
    window: Query<&Window, With<PrimaryWindow>>,
    character: Query<&Transform, With<PlayerMarker>>,
    time: Res<Time>,
) {
    let window = window.single();

    let (mut camera_transform, camera_global, camera, mut director): (_, _, &Camera, _) =
        camera.single_mut();
    let character_transform = character.single();

    if let Some(Vec2 { x, y }) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_global, cursor))
    {
        director.desired_location = (character_transform.translation + Vec3::new(x, y, 0.)) / 2.;
    }

    let dt = time.delta_seconds();
    camera_transform.translation = camera_transform.translation * (1. - dt * director.follow_speed)
        + director.desired_location * dt * director.follow_speed;
}
