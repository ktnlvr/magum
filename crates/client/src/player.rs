use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::camera::CameraOptions;

#[derive(Debug, Default, Clone, Copy, Component, PartialEq, Hash, Reflect)]
pub struct PlayerMarker;

#[derive(Debug, Default, Clone, Copy, Component, PartialEq, Hash, Reflect)]
pub struct PlayerWeaponPivotMarker;

#[derive(Debug, Default, Clone, Copy, Component, PartialEq, Hash, Reflect)]
pub struct PlayerWeaponMarker;

#[derive(Debug, Default, Clone, Copy, Component, PartialEq, Hash, Reflect)]
pub struct PlayerSpriteMarker;

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
            velocity: Vec2::ZERO,
            wish_direction: Vec2::ZERO,
            drag: 9.,
            max_speed: 55.,
            max_accel: 6.,
        }
    }
}

pub fn handle_player_movement(
    mut character: Query<(&mut PlayerMotor, &mut Velocity), With<PlayerMarker>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut motor, mut vel) = character.single_mut();
    let deceleration = (1. + motor.drag * time.delta_seconds()).clamp(0., f32::INFINITY);
    motor.velocity /= deceleration;

    motor.wish_direction.y = keys.pressed(KeyCode::W).then_some(1.).unwrap_or_default()
        + keys.pressed(KeyCode::S).then_some(-1.).unwrap_or_default();
    motor.wish_direction.x = keys.pressed(KeyCode::D).then_some(1.).unwrap_or_default()
        + keys.pressed(KeyCode::A).then_some(-1.).unwrap_or_default();
    motor.wish_direction = motor.wish_direction.normalize_or_zero() * 1.;
    let wish_vector = motor.wish_direction;

    let current_speed = motor.velocity.dot(motor.wish_direction);
    let add_speed = (motor.max_speed - current_speed).clamp(0., motor.max_accel);

    motor.velocity += add_speed * wish_vector;
    vel.linvel = motor.velocity;
}

pub fn animate_player_sprite(
    player: Query<(&PlayerMotor,), Without<PlayerSpriteMarker>>,
    mut sprite: Query<&mut Transform, With<PlayerSpriteMarker>>,
    time: Res<Time>,
    director: Res<CameraOptions>,
) {
    let (motor,) = player.single();
    let mut sprite_transform = sprite.single_mut();

    let bob_intensity =
        motor.velocity.length() / motor.max_speed + motor.velocity.y.abs() / motor.max_speed;

    sprite_transform.translation.y =
        (time.elapsed_seconds() * 27.5).sin() * bob_intensity * director.character_bob_intensity;
}

pub fn animate_player_weapon(
    mut weapon_pivot: Query<(&GlobalTransform, &mut Transform), With<PlayerWeaponPivotMarker>>,
    camera: Query<(&GlobalTransform, &Camera), Without<PlayerMarker>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let (global_pivot, mut local_pivot,) = weapon_pivot.single_mut();
    let (camera_global, camera) = camera.single();
    let window = window.single();

    if let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_global, cursor))
    {
        let direction = cursor_pos - Vec2::new(global_pivot.translation().x, global_pivot.translation().y);
        let angle = direction.y.atan2(direction.x);
        local_pivot.rotation = Quat::from_euler(EulerRot::XYZ, 0., 0., angle);

        local_pivot.scale.y = (direction.x < 0.).then_some(-1.).unwrap_or(1.);
    }
}
