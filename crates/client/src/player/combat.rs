use bevy::{math::vec2, prelude::*, reflect::Reflect, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::{
    core::{DealDamageEvent},
    player::PlayerMarker,
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerAttackEvent>()
            .add_event::<EntityHitEvent>()
            .add_systems(
                Update,
                (attack_input_system, attack_provider, convert_hits_to_damage),
            );
    }
}

#[derive(Debug, Event, Reflect)]
pub struct PlayerAttackEvent {
    player_entity: Entity,
    player_pos: Vec2,
    direction: Vec2,
}

#[derive(Debug, Event, Reflect)]
pub struct EntityHitEvent {
    entity: Entity,
}

pub fn attack_input_system(
    player: Query<(Entity, &GlobalTransform), With<PlayerMarker>>,
    inputs: Res<Input<MouseButton>>,
    mut event_queue: EventWriter<PlayerAttackEvent>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&GlobalTransform, &Camera), Without<PlayerMarker>>,
) {
    let (player_entity, player_transform) = player.single();
    let (global_transform, camera) = camera.single();
    let window = window.single();

    if inputs.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(global_transform, cursor))
        {
            event_queue.send(PlayerAttackEvent {
                player_entity,
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

pub fn attack_provider(
    rapier_ctx: Res<RapierContext>,
    mut attack_events: EventReader<PlayerAttackEvent>,
    mut hit_events: EventWriter<EntityHitEvent>,
) {
    for PlayerAttackEvent {
        player_pos,
        direction,
        player_entity,
    } in attack_events.into_iter()
    {
        if let Some((entity, _hit)) = rapier_ctx.cast_shape(
            *player_pos,
            0.,
            *direction,
            &Collider::ball(1.),
            8.,
            QueryFilter::new()
                .exclude_collider(*player_entity)
                .exclude_sensors(),
        ) {
            hit_events.send(EntityHitEvent { entity })
        }
    }
}

pub fn convert_hits_to_damage(
    mut hit_events: EventReader<EntityHitEvent>,
    mut damage_events: EventWriter<DealDamageEvent>,
) {
    for EntityHitEvent { entity } in hit_events.into_iter() {
        damage_events.send(DealDamageEvent {
            damage: 1,
            target: *entity,
        })
    }
}
