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
        .insert_resource(Score(0))
        .add_systems(Startup, startup)
        .add_systems(Update, player_movement)
        .add_systems(Update, (change_direction))
        .run();
}

#[derive(Component)]
struct Player {
    y: f32,               // x position
    shooting: bool,       // is the player shooting
    remaining_shots: u32, // how many shots the player has left
}

#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

#[derive(Component)]
enum Direction {
    Left,
    Right,
    None
}

#[derive(Component)]
struct Scoreboard;


fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteBundle {
        texture: asset_server.load("player.png"),
        transform: Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
        ..default()
    },Direction::Right));

    commands.spawn((
        Scoreboard,
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: 12.0
                    ,
                    color: Color::srgb(0.5, 0.5, 1.0),
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 12.0,
                color: Color::srgb(1.0, 0.5, 0.5),
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    ));

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

fn change_direction<S:Component>(mut query: Query<(&mut Direction, &mut Transform)>) {
    
    let mut direction = query.single_mut().0;

    direction = Mut::new(Direction::Right);


}