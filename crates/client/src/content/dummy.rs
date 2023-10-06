use bevy::prelude::*;

use crate::core::HealthPool;

#[derive(Component)]
pub struct Dummy;

pub fn tick_dummy_sprite(mut dummies: Query<(&mut HealthPool, &Dummy, &mut TextureAtlasSprite)>) {
    for (hp, _dummy, mut sprite) in dummies.iter_mut() {
        if hp.just_died {
            print!("toast");
            sprite.index += 1;
            sprite.color *= 0.6;
        }
    }
}
