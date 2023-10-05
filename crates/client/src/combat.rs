use bevy::{
    math::{cubic_splines::CubicCurve, vec2},
    prelude::*,
    reflect::Reflect,
    window::PrimaryWindow,
};
use lazy_static::lazy_static;

use crate::player::PlayerMarker;

pub const ATTACK_ANIMATION_DURATION: f32 = 0.23;
const ATTACK_CURVE_CONTROL_POINTS: [[Vec2; 4]; 1] =
    [[vec2(1., 0.), vec2(0.25, 1.105), vec2(0., 1.), vec2(0., 0.)]];

lazy_static! {
    pub static ref ATTACK_CURVE: CubicCurve<Vec2> =
        Bezier::new(ATTACK_CURVE_CONTROL_POINTS).to_curve();
}

#[derive(Debug, Component, Reflect)]
pub struct WeaponMotor {
    pub attack_range: f32,
    pub pivot: Vec2,
    pub timer: Timer,
}

#[derive(Debug, Event, Reflect)]
pub struct PlayerMeleeAttackEvent {
    player_pos: Vec2,
    direction: Vec2,
}

impl Default for WeaponMotor {
    fn default() -> Self {
        Self {
            attack_range: 5.,
            pivot: Vec2::new(4., -2.),
            timer: Timer::default(),
        }
    }
}

pub fn attack_input_system(
    player: Query<(&GlobalTransform,), With<PlayerMarker>>,
    inputs: Res<Input<MouseButton>>,
    mut event_queue: EventWriter<PlayerMeleeAttackEvent>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&GlobalTransform, &Camera), Without<PlayerMarker>>,
) {
    let (player_transform,) = player.single();
    let (global_transform, camera) = camera.single();
    let window = window.single();

    if inputs.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(global_transform, cursor))
        {
            event_queue.send(PlayerMeleeAttackEvent {
                player_pos: vec2(
                    global_transform.translation().x,
                    global_transform.translation().y,
                ),
                direction: (cursor_pos
                    - vec2(
                        player_transform.translation().x,
                        player_transform.translation().y,
                    )),
            })
        }
    }
}

pub fn animate_player_attack(
    mut weapon: Query<(&mut Transform, &mut WeaponMotor)>,
    mut events: EventReader<PlayerMeleeAttackEvent>,
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
