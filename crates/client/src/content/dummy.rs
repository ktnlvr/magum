use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    core::{DamageTakenEvent, HealthPool},
    Animator, AnimatorStateMachine, DUMMY, DUMMY_BROKEN,
};

#[derive(Component, Default)]
pub struct DummyBehaviour;

#[derive(Bundle)]
pub struct DummyBodyBundle {
    pub behaviour: DummyBehaviour,
    pub hp: HealthPool,
    pub collider: Collider,

    #[bundle()]
    pub transform: TransformBundle,
    #[bundle()]
    pub visibility: VisibilityBundle,
}

impl Default for DummyBodyBundle {
    fn default() -> Self {
        Self {
            hp: HealthPool::new(10),
            behaviour: DummyBehaviour::default(),
            visibility: VisibilityBundle::default(),
            transform: TransformBundle::default(),
            collider: Collider::ball(4.),
        }
    }
}

#[derive(Component, Default, Clone, Copy)]
pub enum DummyAnimationState {
    #[default]
    Idle,
    Damaged {
        relative_blow_direction: Vec2,
    },
}

impl AnimatorStateMachine for DummyAnimationState {
    fn calculate_transform(&self, t: f32) -> Transform {
        match self {
            DummyAnimationState::Idle => Transform::IDENTITY,
            DummyAnimationState::Damaged {
                relative_blow_direction,
            } => {
                let mut transform = Transform::IDENTITY;
                transform.rotate_z(
                    relative_blow_direction.x.signum() * 0.75 * (1. - t) * (2. * PI * t).cos()
                        / (t + 1.).powi(2),
                );
                transform.translation -= (*relative_blow_direction * (1. - t).powi(2)).extend(0.);
                transform
            }
        }
    }

    fn duration(&self) -> Duration {
        match self {
            Self::Damaged { .. } => Duration::from_secs_f32(0.6),
            _ => Duration::ZERO,
        }
    }
}

#[derive(Bundle)]
pub struct DummySpriteBundle {
    pub animator: Animator<DummyAnimationState>,

    #[bundle()]
    pub spritesheet: SpriteSheetBundle,
}

impl DummySpriteBundle {
    pub fn new(texture_atlas: Handle<TextureAtlas>) -> Self {
        Self {
            animator: Animator::default(),
            spritesheet: SpriteSheetBundle {
                texture_atlas: texture_atlas.clone(),
                sprite: DUMMY.clone(),
                transform: Transform::IDENTITY,
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct DummyCorpseBundle {
    pub collider: Collider,

    #[bundle()]
    pub spritesheet: SpriteSheetBundle,
}

pub fn dummy_damage_shake(
    mut dummy_animators: Query<(
        &Parent,
        &GlobalTransform,
        &mut Animator<DummyAnimationState>,
    )>,
    mut events: EventReader<DamageTakenEvent>,
) {
    for (parent, global_transform, mut animator) in dummy_animators.iter_mut() {
        for DamageTakenEvent {
            taken_by,
            from_position,
            ..
        } in events.into_iter()
        {
            if parent.get() == *taken_by {
                animator.transition_into(DummyAnimationState::Damaged {
                    relative_blow_direction: (global_transform.translation().truncate()
                        + *from_position)
                        .normalize_or_zero(),
                });
            }
        }
    }
}

pub fn tick_dummy_sprite(
    mut dummies: Query<(Entity, &HealthPool, &Transform, &Children), With<DummyBehaviour>>,
    dummy_sprites: Query<&Handle<TextureAtlas>>,
    mut commands: Commands,
) {
    for (entt, hp, transform, children) in dummies.iter_mut() {
        if hp.just_died {
            let atlas = dummy_sprites
                .get(*children.get(0).unwrap())
                .unwrap()
                .clone();

            commands.entity(entt).despawn_recursive();
            commands.spawn(DummyCorpseBundle {
                spritesheet: SpriteSheetBundle {
                    sprite: DUMMY_BROKEN.clone(),
                    texture_atlas: atlas.clone(),
                    transform: *transform,
                    ..Default::default()
                },
                collider: Collider::ball(4.),
            });
        }
    }
}
