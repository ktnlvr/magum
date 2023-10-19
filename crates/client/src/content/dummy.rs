use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{core::HealthPool, DUMMY, DUMMY_BROKEN};

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
            collider: Collider::ball(4.),
            hp: HealthPool::new(10),
            behaviour: DummyBehaviour::default(),
            visibility: VisibilityBundle::default(),
            transform: TransformBundle::default(),
        }
    }
}

#[derive(Bundle)]
pub struct DummySpriteBundle {
    #[bundle()]
    pub spritesheet: SpriteSheetBundle,
}

impl DummySpriteBundle {
    pub fn new(texture_atlas: Handle<TextureAtlas>) -> Self {
        Self {
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

#[derive(Component, Default)]
pub struct DummyBehaviour;

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
