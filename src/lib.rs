use bevy::{
    camera::ScalingMode,
    color::palettes::tailwind::{CYAN_300, RED_600},
    input::keyboard::Key,
    prelude::*,
};

pub mod pipes;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Gravity(f32);

impl Default for Gravity {
    fn default() -> Self {
        Gravity(DEFAULT_GRAVITY)
    }
}

#[derive(Component, Default)]
pub struct Velocity(f32);

#[derive(Component)]
pub struct TopPipe;

#[derive(Component)]
pub struct BottomPipe;

#[derive(Event)]
pub struct GameOver;

#[derive(Component)]
pub struct ScoreText;

const MAX_WIDTH: f32 = 640.0;
const MAX_HEIGHT: f32 = 360.0;
const DEFAULT_GRAVITY: f32 = -MAX_HEIGHT / 8.0;
const VELOCITY_BOOST: f32 = MAX_HEIGHT / 8.0;
const PLAYER_HALF_HEIGHT: f32 = 25.0;
const PLAYER_HALF_WIDTH: f32 = 50.0;
const PLAYER_X_POS: f32 = -MAX_WIDTH / 4.0;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(ClearColor(Color::from(CYAN_300)));

    // spawn the camera
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: MAX_WIDTH,
                max_height: MAX_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    // let's show a simple ellipse
    let ellipse = Mesh2d(meshes.add(Ellipse::new(PLAYER_HALF_WIDTH, PLAYER_HALF_HEIGHT)));
    let material = MeshMaterial2d(materials.add(Color::from(RED_600)));

    commands.spawn((
        Player,
        Velocity::default(),
        Gravity::default(),
        ellipse,
        material,
        Transform::from_xyz(PLAYER_X_POS, 0.0, 1.0),
    ));

    commands.add_observer(reset_game);

    commands.spawn((
        ScoreText,
        Text::new("0"),
        Transform::from_xyz(0.0, 0.0, 0.0),
        TextColor(Color::BLACK),
    ));
}

/// The gravity system
pub fn gravity(mut physics: Query<(&mut Transform, &mut Velocity, &Gravity)>, time: Res<Time>) {
    for (mut transform, mut velocity, gravity) in &mut physics {
        // Δx = vt
        transform.translation.y += velocity.0 * time.delta_secs();
        // Δv = at
        velocity.0 += gravity.0 * time.delta_secs();
    }
}

/// Handle user input
pub fn handle_input(
    mut velocity: Single<&mut Velocity, With<Player>>,
    keys: Res<ButtonInput<Key>>,
) {
    if keys.just_pressed(Key::Space) {
        velocity.0 += VELOCITY_BOOST;
    }
}

/// Detect when the player goes out of bounds
pub fn check_out_of_bounds(transform: Single<&Transform, With<Player>>, mut commands: Commands) {
    let Vec3 { y, .. } = transform.translation;
    let bottom = y - PLAYER_HALF_HEIGHT;
    let top = y + PLAYER_HALF_HEIGHT;

    if bottom < -MAX_HEIGHT / 2.0 || top > MAX_HEIGHT / 2.0 {
        commands.trigger(GameOver);
    }
}

fn reset_game(
    _: On<GameOver>,
    mut commands: Commands,
    player: Single<Entity, With<Player>>,
    pipes: Query<Entity, Or<(With<TopPipe>, With<BottomPipe>)>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    // can trigger a second event to reset the pipes for better encapsulation
    commands.entity(*player).insert((
        Velocity::default(),
        Transform::from_xyz(PLAYER_X_POS, 0.0, 1.0),
    ));
    for entity in pipes {
        commands.entity(entity).despawn();
    }
    pipes::spawn_pipes(commands, meshes, materials);
}
