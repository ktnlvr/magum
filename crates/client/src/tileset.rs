use bevy::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CHECKERBOARD: TextureAtlasSprite = TextureAtlasSprite {
        index: 0,
        ..default()
    };
    pub static ref WALL: TextureAtlasSprite = TextureAtlasSprite {
        color: Color::rgb_u8(0x64, 0x6C, 0x5E),
        index: 1,
        ..default()
    };
    pub static ref CHEST: TextureAtlasSprite = TextureAtlasSprite {
        index: 2,
        ..default()
    };
    pub static ref ROCK: TextureAtlasSprite = TextureAtlasSprite {
        color: Color::rgb_u8(0x64, 0x6C, 0x5E),
        index: 3,
        ..default()
    };
    pub static ref DUMMY: TextureAtlasSprite = TextureAtlasSprite {
        color: Color::rgb_u8(0x7D, 0x5C, 0x51),
        index: 17,
        ..default()
    };
    pub static ref DUMMY_BROKEN: TextureAtlasSprite = TextureAtlasSprite {
        color: Color::rgb_u8(0x7D, 0x5C, 0x51) * 0.7,
        index: 18,
        ..default()
    };
    pub static ref GRASS: TextureAtlasSprite = TextureAtlasSprite {
        color: Color::rgb_u8(0x48, 0x4A, 0x16),
        index: 4,
        ..default()
    };
    pub static ref PLAYER: TextureAtlasSprite = TextureAtlasSprite {
        color: Color::rgb_u8(0xFF, 0xFF, 0xFF),
        index: 16,
        ..default()
    };
    pub static ref SWORD: TextureAtlasSprite = TextureAtlasSprite {
        color: Color::rgb_u8(0x91, 0x87, 0x83),
        index: 13,
        ..default()
    };
}
