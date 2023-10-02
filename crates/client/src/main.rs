use bevy::{prelude::*, window::PrimaryWindow};
use hero::{CameraBundle, CameraDirector, HeroBundle, IsPlayer, PlayerMarker, PlayerMotor};

mod hero;

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

    commands.spawn((HeroBundle {
        marker: PlayerMarker,
        motor: PlayerMotor {
            ..Default::default()
        },
        sprite: SpriteSheetBundle {
            texture_atlas,
            sprite: TextureAtlasSprite::new(16),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    },));
}

pub fn player_motor(
    mut character: Query<(&mut PlayerMotor, &mut Transform), IsPlayer>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut motor, mut transform) = character.single_mut();
    let deceleration = (1. + motor.drag * time.delta_seconds()).clamp(0., f32::INFINITY);
    motor.velocity /= deceleration;

    motor.wish_direction.y = keys.pressed(KeyCode::W).then_some(1.).unwrap_or_default()
        + keys.pressed(KeyCode::S).then_some(-1.).unwrap_or_default();
    motor.wish_direction.x = keys.pressed(KeyCode::D).then_some(1.).unwrap_or_default()
        + keys.pressed(KeyCode::A).then_some(-1.).unwrap_or_default();
    motor.wish_direction = motor.wish_direction.normalize_or_zero() * 1.;

    let current_speed = motor.velocity.dot(motor.wish_direction);
    let add_speed =
        (motor.max_speed - current_speed).clamp(0., motor.max_accel * time.delta_seconds());
    motor.velocity = motor.velocity + add_speed * motor.wish_direction;

    transform.translation += Vec3::new(motor.velocity.x, motor.velocity.y, 0.);
}

pub fn move_camera(
    mut camera: Query<
        (
            &mut Transform,
            &GlobalTransform,
            &Camera,
            &mut CameraDirector,
        ),
        Without<PlayerMarker>,
    >,
    window: Query<&Window, With<PrimaryWindow>>,
    character: Query<&Transform, IsPlayer>,
    time: Res<Time>,
) {
    let window = window.single();

    let (mut camera_transform, camera_global, camera, mut director): (_, _, &Camera, _) =
        camera.single_mut();
    let character_transform = character.single();

    if let Some(Vec2 { x, y }) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_global, cursor))
    {
        director.desired_location = (character_transform.translation + Vec3::new(x, y, 0.)) / 2.;
    }

    let dt = time.delta_seconds();
    camera_transform.translation = camera_transform.translation * (1. - dt * director.follow_speed)
        + director.desired_location * dt * director.follow_speed;
}

pub fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(0x0A, 0x0D, 0x11)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_motor, move_camera))
        .run();
}
