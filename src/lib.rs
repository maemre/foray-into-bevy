use bevy::{
    camera::ScalingMode,
    color::palettes::tailwind::{GREEN_600, RED_600},
    input::keyboard::Key,
    prelude::*,
};

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

const MAX_WIDTH: f32 = 640.0;
const MAX_HEIGHT: f32 = 360.0;
const DEFAULT_GRAVITY: f32 = -MAX_HEIGHT / 2.0;
const VELOCITY_BOOST: f32 = MAX_HEIGHT / 4.0;
const PLAYER_HALF_HEIGHT: f32 = 25.0;
const PLAYER_HALF_WIDTH: f32 = 50.0;
const PLAYER_X_POS: f32 = -MAX_WIDTH / 4.0;
const PIPE_START: f32 = 0.0;
const PIPE_END: f32 = MAX_WIDTH / 2.0 + PIPE_HALF_WIDTH;
const NUM_PIPES: usize = 3;
const PIPE_HALF_WIDTH: f32 = 50.0;
const PIPE_GAP: f32 = 150.0;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
}

pub fn spawn_pipes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material = MeshMaterial2d(materials.add(Color::from(GREEN_600)));

    for i in 0..NUM_PIPES {
        // calculate the center x coordinate
        let center_x = PIPE_START + (PIPE_END - PIPE_START) * i as f32 / (NUM_PIPES - 1) as f32;
        info!("spawning pipe {i} at x coordinate: {center_x}");

        let rect = Mesh2d(meshes.add(Rectangle::new(PIPE_HALF_WIDTH, MAX_HEIGHT - PIPE_GAP)));
        // spawn the top pipe
        commands.spawn((
            TopPipe,
            rect.clone(),
            material.clone(),
            Transform::from_xyz(center_x, MAX_HEIGHT / 2.0, 1.0),
        ));

        // spawn the bottom pipe
        commands.spawn((
            BottomPipe,
            rect,
            material.clone(),
            Transform::from_xyz(center_x, -MAX_HEIGHT / 2.0, 1.0),
        ));
    }
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
pub fn check_out_of_bounds(
    transform: Single<&Transform, With<Player>>,
    mut exit: MessageWriter<AppExit>,
) {
    let Vec3 { y, .. } = transform.translation;
    let bottom = y - PLAYER_HALF_HEIGHT;
    let top = y + PLAYER_HALF_HEIGHT;

    if bottom < -MAX_HEIGHT / 2.0 || top > MAX_HEIGHT * 2.0 {
        exit.write(AppExit::Success);
    }
}
