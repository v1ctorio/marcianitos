use bevy::{prelude::*, window::WindowTheme};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Marcianitos".into(),
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, startup)
        .add_systems(Update, player_movement)
        .run();
}

#[derive(Component)]
struct Player {
    x: f32,               // x position
    shooting: bool,       // is the player shooting
    remaining_shots: u32, // how many shots the player has left
}

#[derive(Component)]
enum Direction {
    Left,
    Right,
    None
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteBundle {
        texture: asset_server.load("player.png"),
        transform: Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
        ..default()
    },Direction::Right));
}

fn player_movement(time: Res<Time>, mut query: Query<(&Direction, &mut Transform)>) {
    for (direction, mut transform) in query.iter_mut() {
        match direction {
            Direction::Left => {
                transform.translation.x -= time.delta_seconds();
            }
            Direction::Right => {
                transform.translation.x += time.delta_seconds();
            }
            _ => {}
        }
    }
}
//fn add_player(mut commands: Commands) {}
