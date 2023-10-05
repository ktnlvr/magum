use bevy::prelude::*;

use crate::{camera::CameraOptions, collision::RigidBody};

#[derive(Debug, Default, Clone, Copy, Component, PartialEq, Hash, Reflect)]
pub struct PlayerMarker;

#[derive(Debug, Default, Clone, Copy, Component, PartialEq, Hash, Reflect)]
pub struct PlayerSpriteMarker;

#[derive(Debug, Clone, Component, Reflect)]
pub struct PlayerMotor {
    pub wish_direction: Vec2,

    pub drag: f32,
    pub max_speed: f32,
    pub max_accel: f32,
}

impl Default for PlayerMotor {
    fn default() -> Self {
        Self {
            wish_direction: Vec2::ZERO,
            drag: 9.,
            max_speed: 55.,
            max_accel: 6.,
        }
    }
}

pub fn handle_player_movement(
    mut character: Query<(&mut PlayerMotor, &mut RigidBody), With<PlayerMarker>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut motor, mut rb) = character.single_mut();
    let deceleration = (1. + motor.drag * time.delta_seconds()).clamp(0., f32::INFINITY);
    rb.velocity /= deceleration;

    motor.wish_direction.y = keys.pressed(KeyCode::W).then_some(1.).unwrap_or_default()
        + keys.pressed(KeyCode::S).then_some(-1.).unwrap_or_default();
    motor.wish_direction.x = keys.pressed(KeyCode::D).then_some(1.).unwrap_or_default()
        + keys.pressed(KeyCode::A).then_some(-1.).unwrap_or_default();
    motor.wish_direction = motor.wish_direction.normalize_or_zero() * 1.;

    let current_speed = rb.velocity.dot(motor.wish_direction);
    let add_speed =
        (motor.max_speed - current_speed).clamp(0., motor.max_accel);

    rb.velocity += add_speed * motor.wish_direction;
}

pub fn animate_player_sprite(
    player: Query<(&RigidBody, &PlayerMotor), Without<PlayerSpriteMarker>>,
    mut sprite: Query<&mut Transform, With<PlayerSpriteMarker>>,
    time: Res<Time>,
    director: Res<CameraOptions>,
) {
    let (rb, motor) = player.single();
    let mut sprite_transform = sprite.single_mut();

    let bob_intensity =
        rb.velocity.length() / motor.max_speed + rb.velocity.y.abs() / motor.max_speed;

    sprite_transform.translation.y =
        (time.elapsed_seconds() * 27.5).sin() * bob_intensity * director.character_bob_intensity;
}
