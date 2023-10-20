use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    core::{DamageTakenEvent, HealthPool},
    Animator, Clip, DUMMY, DUMMY_BROKEN,
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

#[derive(Component, Default)]
pub struct DummySpriteMarker;

#[derive(Bundle)]
pub struct DummySpriteBundle {
    pub animator: Animator,

    #[bundle()]
    pub spritesheet: SpriteSheetBundle,
    _marker: DummySpriteMarker,
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
            _marker: DummySpriteMarker::default(),
        }
    }
}

#[derive(Bundle)]
pub struct DummyCorpseBundle {
    pub collider: Collider,

    #[bundle()]
    pub spritesheet: SpriteSheetBundle,
}

struct DummyShakeAnimationClip;

impl Clip for DummyShakeAnimationClip {
    fn animate(&self, time_normalized: f32) -> Transform {
        Transform::from_scale(Vec3::ONE * time_normalized)
    }
}

pub fn dummy_damage_shake(
    mut dummy_animators: Query<(&Parent, &mut Animator), With<DummySpriteMarker>>,
    mut events: EventReader<DamageTakenEvent>,
) {
    for (dummy_sprite_parent, mut dummy_animator) in dummy_animators.iter_mut() {
        for DamageTakenEvent { taken_by, .. } in events.into_iter() {
            if dummy_sprite_parent.get() == *taken_by {
                dummy_animator.play(DummyShakeAnimationClip, Duration::from_secs(1));
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
