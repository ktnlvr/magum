use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use crate::player::PlayerMarker;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraOptions>()
            .init_resource::<CameraOptions>()
            .register_type::<CameraMotor>()
            .add_systems(Update, (move_camera,));
    }
}

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct CameraOptions {
    pub follow_speed: f32,
    pub character_to_cursor_center: f32,
    pub character_bob_intensity: f32,
}

impl Default for CameraOptions {
    fn default() -> Self {
        Self {
            follow_speed: 30.,
            character_to_cursor_center: 0.25,
            character_bob_intensity: 0.75,
        }
    }
}

#[derive(Component, Reflect)]
pub struct CameraMotor {
    pub desired_location: Vec3,
}

impl Default for CameraMotor {
    fn default() -> Self {
        Self {
            desired_location: Vec3::ZERO,
        }
    }
}

#[derive(Bundle)]
pub struct CameraBundle {
    pub director: CameraMotor,
    pub camera: Camera2dBundle,
}

impl Default for CameraBundle {
    fn default() -> Self {
        Self {
            director: CameraMotor::default(),

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
        (&mut Transform, &GlobalTransform, &Camera, &mut CameraMotor),
        Without<PlayerMarker>,
    >,
    window: Query<&Window, With<PrimaryWindow>>,
    character: Query<&Transform, With<PlayerMarker>>,
    time: Res<Time>,
    director: Res<CameraOptions>,
) {
    let window = window.single();

    let (mut camera_transform, camera_global, camera, mut motor): (_, _, &Camera, _) =
        camera.single_mut();
    let character_transform = character.single();

    if let Some(Vec2 { x, y }) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_global, cursor))
    {
        motor.desired_location = character_transform.translation
            * (1. - director.character_to_cursor_center)
            + Vec3::new(x, y, 0.) * (director.character_to_cursor_center);
    }

    let dt = time.delta_seconds();
    camera_transform.translation = camera_transform.translation * (1. - dt * director.follow_speed)
        + motor.desired_location * dt * director.follow_speed;
}
