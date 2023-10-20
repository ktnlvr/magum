use std::time::Duration;

use crate::{animator_system, Animator, AnimatorStateMachine};

use super::{CameraOptions, PlayerAttackEvent, PlayerMarker, PlayerMotor};
use bevy::{
    math::{cubic_splines::CubicCurve, vec2},
    prelude::*,
    window::PrimaryWindow,
};
use lazy_static::lazy_static;

pub struct PlayerAnimatorPlugin;

impl Plugin for PlayerAnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animate_player_sprite,
                animate_player_attack,
                animate_player_weapon,
                animator_system::<WeaponAnimationState>,
            ),
        );
    }
}

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

#[derive(Debug, Clone)]
pub struct WeaponAnimationState {
    pub look_direction: Vec2,
    pub weapon_pivot: Vec2,
    state: WeaponAnimationStateState,
}

impl Default for WeaponAnimationState {
    fn default() -> Self {
        Self {
            look_direction: Default::default(),
            weapon_pivot: Vec2::new(0., -1.5),
            state: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum WeaponAnimationStateState {
    #[default]
    Idle,
    Attacking {
        range: f32,
    },
}

impl AnimatorStateMachine for WeaponAnimationState {
    fn calculate_transform(&self, t: f32) -> Transform {
        use WeaponAnimationStateState::*;

        let mut offset = Transform::IDENTITY;
        offset.rotate_z(self.look_direction.y.atan2(self.look_direction.x));
        offset.scale.y = self.look_direction.x.signum();
        offset.translation = (self.weapon_pivot
            + Vec2::new(4., 0.).rotate(self.look_direction.normalize()))
        .extend(0.);

        offset
            * match self.state {
                Idle => Transform::IDENTITY,
                Attacking { range } => {
                    Transform::from_xyz(ATTACK_CURVE.position(t).y * range, 0., 0.)
                }
            }
    }

    fn duration(&self) -> std::time::Duration {
        match self.state {
            WeaponAnimationStateState::Idle => Duration::ZERO,
            WeaponAnimationStateState::Attacking { .. } => {
                Duration::from_secs_f32(ATTACK_ANIMATION_DURATION)
            }
        }
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
    mut weapon_animator: Query<&mut Animator<WeaponAnimationState>>,
    weapon_pivot: Query<&GlobalTransform, With<PlayerMarker>>,
    camera: Query<(&GlobalTransform, &Camera), Without<PlayerMarker>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let mut animator = weapon_animator.single_mut();
    let (camera_global, camera) = camera.single();
    let weapon_pivot = weapon_pivot.single();
    let window = window.single();

    if let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_global, cursor))
    {
        let direction = cursor_pos - weapon_pivot.translation().truncate();
        animator.mutate_state(|state| state.look_direction = direction.clamp_length(0., 1.));
    }
}

pub fn animate_player_attack(
    mut weapon: Query<&mut Animator<WeaponAnimationState>>,
    mut events: EventReader<PlayerAttackEvent>,
) {
    let mut weapon = weapon.single_mut();

    if events.iter().next().is_some() {
        weapon.transition_into(WeaponAnimationState {
            state: WeaponAnimationStateState::Attacking { range: 5. },
            ..WeaponAnimationState::default()
        });
    }
}
