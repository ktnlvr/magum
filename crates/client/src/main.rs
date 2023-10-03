use bevy::prelude::*;
use bevy_inspector_egui::{quick::WorldInspectorPlugin, DefaultInspectorConfigPlugin};
use camera::{move_camera, CameraBundle, CameraMotor, CameraOptions};
use hero::HeroBundle;
use player::{animate_player_sprite, move_player, PlayerMotor, PlayerSpriteMarker};

mod camera;
mod hero;
mod player;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("tileset.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 16, 16, None, None);
    let texture_atlas = atlases.add(texture_atlas);

    commands.spawn(CameraBundle::default());

    let mut grass_sprite = TextureAtlasSprite::new(4);
    grass_sprite.color = Color::rgb_u8(0x48, 0x4A, 0x16);

    let mut rock_sprite = TextureAtlasSprite::new(3);
    rock_sprite.color = Color::rgb_u8(0x64, 0x6C, 0x5E);

    let mut player_sprite = TextureAtlasSprite::new(16);
    player_sprite.color = Color::rgb_u8(0xC8, 0xAC, 0x5E);

    let mut wall_sprite = TextureAtlasSprite::new(1);
    wall_sprite.color = Color::rgb_u8(0x64, 0x6C, 0x5E);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas.clone(),
        sprite: grass_sprite.clone(),
        transform: Transform::from_xyz(-16., 8., 0.),
        ..default()
    });
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas.clone(),
        sprite: grass_sprite.clone(),
        transform: Transform::from_xyz(12., 4., 0.),
        ..default()
    });
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas.clone(),
        sprite: grass_sprite.clone(),
        transform: Transform::from_xyz(4., -8., 0.),
        ..default()
    });
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas.clone(),
        sprite: grass_sprite.clone(),
        transform: Transform::from_xyz(-4., -12., 0.),
        ..default()
    });

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas.clone(),
        sprite: rock_sprite.clone(),
        transform: Transform::from_xyz(-8., 12., 0.),
        ..default()
    });

    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas.clone(),
        sprite: wall_sprite.clone(),
        transform: Transform::from_xyz(16., 16., 0.),
        ..default()
    },));

    commands
        .spawn((HeroBundle {
            ..Default::default()
        },))
        .with_children(|hero| {
            hero.spawn((
                SpriteSheetBundle {
                    texture_atlas,
                    sprite: TextureAtlasSprite::new(16),
                    transform: Transform::from_xyz(0., 0., 0.),
                    ..default()
                },
                PlayerSpriteMarker,
            ));
        });
}

pub fn main() {
    App::new()
        .register_type::<CameraOptions>()
        .register_type::<PlayerMotor>()
        .register_type::<CameraMotor>()
        .insert_resource(ClearColor(Color::rgb_u8(0x0A, 0x0D, 0x11)))
        .init_resource::<CameraOptions>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, move_camera, animate_player_sprite))
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
