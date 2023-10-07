use bevy::prelude::*;

use crate::core::HealthPool;

#[derive(Component, Default)]
pub struct DummyBehaviour;

pub fn tick_dummy_sprite(
    mut dummies: Query<(&HealthPool, &mut TextureAtlasSprite), With<DummyBehaviour>>,
) {
    for (hp, mut sprite) in dummies.iter_mut() {
        if hp.just_died {
            sprite.index += 1;
            sprite.color *= 0.7;
        }
    }
}
