use bevy::{
    prelude::*,
    // sprite::collide_aabb::{collide, Collision},
    window::{PresentMode, WindowResized},
};

const BASICALLY_ZERO: f32 = 0.001;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: String::from("Harvest Game"),
                width: 1280.0,
                height: 720.0,
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

    let window = windows.primary();

    let player_texture = asset_server.load("player.png");
    let floor_texture = asset_server.load("ohno.png");

    let size = window.width() / 16.0;

    let tiles: Vec<Vec<Tile>> = (0..9)
        .map(|row| {
            (0..16)
                .map(|col| Tile {
                    floor: commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(size, size)),
                                ..default()
                            },
                            texture: floor_texture.clone(),
                            transform: Transform {
                                translation: Vec3::new(
                                    size * (col as f32 - 7.5),
                                    size * (row - 4) as f32,
                                    0.0,
                                ),
                                ..default()
                            },
                            ..default()
                        })
                        .id(),
                    block: commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(size, size)),
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3::new(
                                    size * (col as f32 - 7.5),
                                    size * (row - 4) as f32,
                                    1.0,
                                ),
                                ..default()
                            },
                            visibility: Visibility::INVISIBLE,
                            ..default()
                        })
                        .id(),
                })
                .collect()
        })
        .collect();

    commands.insert_resource(Game { tiles: tiles });

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            texture: player_texture,
            transform: Transform {
                translation: Vec2::ZERO.extend(1.0),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Velocity(Vec3::default()))
        .insert(Damping(2.0));
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

fn place_blocks(
    // mut query: Query<(&Transform, &Sprite, &mut Handle<Image>), With<Block>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Res<Game>,
    windows: Res<Windows>,
    mouse: Res<Input<MouseButton>>,
) {
    if mouse.pressed(MouseButton::Left) {
        let window = windows.primary();

        if let Some(pos) = window.cursor_position() {
            let texture: Handle<Image> = asset_server.load("player.png");
            let size = window.width() / 16.0;
            let row = (pos.y / size) as usize;
            let col = (pos.x / size) as usize;

            if row >= game.tiles.len() || col >= game.tiles[0].len() {
                return;
            } 

            commands
                .entity(game.tiles[row][col].block)
                .insert(texture)
                .insert(Visibility::VISIBLE);
        }
    }
}

fn damping_system(mut query: Query<(&mut Velocity, &Damping)>) {
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

// enum FloorType {
//     Ground,
// }

// enum BlockType {
//     Air,
// }

// #[derive(Component)]
// struct Floor(FloorType);

// #[derive(Component)]
// struct Block(BlockType);

#[derive(Component)]
struct Player;

struct Tile {
    floor: Entity,
    block: Entity,
}

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Damping(f32);

#[derive(Resource)]
struct Game {
    // [row][col]
    tiles: Vec<Vec<Tile>>,
}

#[derive(Resource)]
struct Settings;

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(player_movement)
            .add_system(place_blocks)
            .add_system(velocity_system)
            .add_system(damping_system);
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
