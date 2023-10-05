use bevy::{math::vec2, prelude::*, reflect::Reflect, window::PrimaryWindow};

use crate::player::PlayerMarker;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMeleeAttackEvent>()
            .add_systems(Update, (attack_input_system,));
    }
}

#[derive(Debug, Event, Reflect)]
pub struct PlayerMeleeAttackEvent {
    player_pos: Vec2,
    direction: Vec2,
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
                    ))
                .normalize_or_zero(),
            })
        }
    }
}
