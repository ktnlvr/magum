use bevy::prelude::*;
use bevy_inspector_egui::{quick::WorldInspectorPlugin, DefaultInspectorConfigPlugin};
use bevy_rapier2d::prelude::*;
use hero::HeroBundle;
use player::{
    CameraBundle, CameraPlugin, PlayerAnimatorPlugin, PlayerLocomotionPlugin,
    PlayerMeleeAttackEvent, PlayerSpriteMarker, PlayerWeaponMarker, PlayerWeaponPivotMarker,
    WeaponAnimator,
};

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

    let mut sword_sprite = TextureAtlasSprite::new(13);
    sword_sprite.color = Color::rgb_u8(0x91, 0x87, 0x83);

    let mut dummy_sprite = TextureAtlasSprite::new(17);
    dummy_sprite.color = Color::rgb_u8(0x7D, 0x5C, 0x51);

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

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas.clone(),
            sprite: wall_sprite.clone(),
            transform: Transform::from_xyz(16., 16., 0.),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(4., 4.),
    ));

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas.clone(),
        sprite: dummy_sprite.clone(),
        transform: Transform::from_xyz(-8., 32., 0.),
        ..default()
    });

    commands
        .spawn((HeroBundle {
            ..Default::default()
        },))
        .with_children(|hero| {
            hero.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas.clone(),
                    sprite: TextureAtlasSprite::new(16),
                    ..default()
                },
                PlayerSpriteMarker,
            ));

            hero.spawn((
                TransformBundle {
                    local: Transform::from_xyz(0., 0., 0.),
                    ..default()
                },
                VisibilityBundle::default(),
                PlayerWeaponPivotMarker,
            ))
            .with_children(|pivot| {
                pivot.spawn((
                    SpriteSheetBundle {
                        texture_atlas,
                        sprite: sword_sprite,
                        transform: Transform::from_xyz(0., 0., 1.),
                        ..default()
                    },
                    PlayerWeaponMarker,
                    WeaponAnimator::default(),
                ));
            });
        });
}

pub fn toggle_debug_render_context(mut ctx: ResMut<DebugRenderContext>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Slash) {
        ctx.enabled = !ctx.enabled;
    }
}

pub fn main() {
    App::new()
        .add_event::<PlayerMeleeAttackEvent>()
        // background
        .insert_resource(ClearColor(Color::rgb_u8(0x0A, 0x0D, 0x11)))
        // builtins
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // game related stuff
        .add_plugins((CameraPlugin, PlayerAnimatorPlugin, PlayerLocomotionPlugin))
        // physics
        .register_type::<RigidBody>()
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0),
            RapierDebugRenderPlugin {
                enabled: false,
                ..Default::default()
            },
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_debug_render_context,))
        // cool gui stuff
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
