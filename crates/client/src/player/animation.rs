use super::{CameraOptions, PlayerAttackEvent, PlayerMarker, PlayerMotor};
use bevy::{
    math::{cubic_splines::CubicCurve, vec2},
    prelude::*,
    window::PrimaryWindow,
};
use lazy_static::lazy_static;

#[derive(Debug, Default, Clone, Copy, Component, PartialEq, Hash, Reflect)]
pub struct PlayerWeaponPivotMarker;

#[derive(Debug, Default, Clone, Copy, Component, PartialEq, Hash, Reflect)]
pub struct PlayerWeaponMarker;

#[derive(Debug, Default, Clone, Copy, Component, PartialEq, Hash, Reflect)]
pub struct PlayerSpriteMarker;

pub const ATTACK_ANIMATION_DURATION: f32 = 0.23;
const ATTACK_CURVE_CONTROL_POINTS: [[Vec2; 4]; 1] =
    [[vec2(1., 0.), vec2(0.25, 1.105), vec2(0., 1.), vec2(0., 0.)]];

lazy_static! {
    pub static ref ATTACK_CURVE: CubicCurve<Vec2> =
        Bezier::new(ATTACK_CURVE_CONTROL_POINTS).to_curve();
}

#[derive(Debug, Component, Reflect)]
pub struct WeaponAnimator {
    pub attack_range: f32,
    pub pivot: Vec2,
    pub timer: Timer,
}

impl Default for WeaponAnimator {
    fn default() -> Self {
        Self {
            attack_range: 5.,
            pivot: Vec2::new(4., -2.),
            timer: Timer::default(),
        }
    }
}

pub struct PlayerAnimatorPlugin;

impl Plugin for PlayerAnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WeaponAnimator>().add_systems(
            Update,
            (
                animate_player_sprite,
                animate_player_weapon,
                animate_player_attack,
            ),
        );
    }
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
    let (global_pivot, mut local_pivot) = weapon_pivot.single_mut();
    let (camera_global, camera) = camera.single();
    let window = window.single();

    if let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_global, cursor))
    {
        let direction = cursor_pos - global_pivot.translation().truncate();
        let angle = direction.y.atan2(direction.x);
        local_pivot.rotation = Quat::from_euler(EulerRot::XYZ, 0., 0., angle);

        local_pivot.scale.y = if direction.x < 0. { -1. } else { 1. };
    }
}

pub fn animate_player_attack(
    mut weapon: Query<(&mut Transform, &mut WeaponAnimator)>,
    mut events: EventReader<PlayerAttackEvent>,
    time: Res<Time>,
) {
    let (mut transform, mut weapon) = weapon.single_mut();

    weapon.timer.tick(time.delta());

    if events.iter().next().is_some() {
        weapon.timer = Timer::from_seconds(ATTACK_ANIMATION_DURATION, TimerMode::Once);
    }

    transform.translation.x =
        ATTACK_CURVE.position(weapon.timer.percent()).y * weapon.attack_range + weapon.pivot.x;
    transform.translation.y = weapon.pivot.y;
}
