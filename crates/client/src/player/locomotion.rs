use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerLocomotionPlugin;

impl Plugin for PlayerLocomotionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerMotor>()
            .add_systems(Update, handle_player_movement);
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

#[derive(Debug, Default, Clone, Copy, Component, PartialEq, Hash, Reflect)]
pub struct PlayerMarker;

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
