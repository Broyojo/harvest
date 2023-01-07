use bevy::{
    prelude::*,
    window::{PresentMode, WindowResized},
};

const BASICALLY_ZERO: f32 = 0.001;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: String::from("Harvest Game"),
                width: 1920.0,
                height: 1080.0,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(GamePlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("player.png");

    commands
        .spawn(SpriteBundle {
            texture: texture_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(Player)
        .insert(Velocity(Vec3::default()))
        .insert(LinearDamping(2.0));
}

fn player_movement(mut query: Query<&mut Velocity, With<Player>>, keys: Res<Input<KeyCode>>) {
    let mut velocity = query.single_mut();
    let speed = 10.0;

    if keys.pressed(KeyCode::W) {
        velocity.0.y += speed;
    }

    if keys.pressed(KeyCode::A) {
        velocity.0.x -= speed;
    }

    if keys.pressed(KeyCode::S) {
        velocity.0.y -= speed;
    }

    if keys.pressed(KeyCode::D) {
        velocity.0.x += speed;
    }
}

fn linear_damping_system(mut query: Query<(&mut Velocity, &LinearDamping)>) {
    for (mut velocity, damping) in query.iter_mut() {
        if velocity.0.length() > BASICALLY_ZERO {
            velocity.0 /= damping.0;
        }
    }
}

fn velocity_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0;
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct LinearDamping(f32);

#[derive(Component)]
struct Tile;

#[derive(Resource)]
struct Settings;

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(player_movement)
            .add_system(velocity_system)
            .add_system(linear_damping_system);
    }
}

#[allow(unused)]
fn window_resize(resize_event: Res<Events<WindowResized>>, mut windows: ResMut<Windows>) {
    let mut event_reader = resize_event.get_reader();
    let window = windows.primary_mut();
    for event in event_reader.iter(&resize_event) {
        window.set_resolution(event.width, event.height);
    }
}
