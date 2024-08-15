use bevy::{input::keyboard::KeyboardInput, prelude::*, window::WindowTheme};


const MOVEMENT_SPEED: f32 = 100.0;

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
        .add_systems(Update, handle_input)
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
    for (direction, mut transform) in &mut query {
        match *direction {
            Direction::Left => {
                transform.translation.x -= MOVEMENT_SPEED * time.delta_seconds();
            }
            Direction::Right => {
                transform.translation.x += MOVEMENT_SPEED * time.delta_seconds();
            }
            _ => {}
        }
    }
}

// doesnt work xdd 
fn handle_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut Direction, &mut Player)>) {
    for (mut direction, mut _player) in &mut query {

        println!("{:?}", keyboard_input.pressed(KeyCode::ArrowLeft));

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            *direction = Direction::Left;
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            *direction = Direction::Right;
        } else {
            *direction = Direction::None;   
        }
    }
}

