use bevy::{
    camera::ScalingMode,
    color::palettes::tailwind::{GREEN_600, RED_600},
    input::keyboard::Key,
    math::bounding::{Bounded2d, BoundingCircle, IntersectsVolume},
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
const DEFAULT_GRAVITY: f32 = -MAX_HEIGHT / 4.0;
const VELOCITY_BOOST: f32 = MAX_HEIGHT / 8.0;
const PLAYER_HALF_HEIGHT: f32 = 25.0;
const PLAYER_HALF_WIDTH: f32 = 50.0;
const PLAYER_X_POS: f32 = -MAX_WIDTH / 4.0;
const PIPE_START: f32 = -MAX_WIDTH / 6.0;
const PIPE_END: f32 = MAX_WIDTH / 2.0 + PIPE_HALF_WIDTH;
const NUM_PIPES: usize = 3;
const PIPE_HALF_WIDTH: f32 = 50.0;
const PIPE_HEIGHT: f32 = MAX_HEIGHT - PIPE_GAP;
const PIPE_GAP: f32 = 150.0;
const PIPE_SPEED: f32 = 100.0;
pub const PIPE_SPAWN_PERIOD: f32 = MAX_WIDTH / (PIPE_SPEED * NUM_PIPES as f32);

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
    for i in 0..NUM_PIPES {
        // calculate the center x coordinate
        let center_x = PIPE_START + (PIPE_END - PIPE_START) * i as f32 / (NUM_PIPES - 1) as f32;
        spawn_pipes_at(center_x, &mut commands, &mut meshes, &mut materials);
    }
}

fn spawn_pipes_at(
    center_x: f32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    info!("spawning pipes at x coordinate: {center_x}");
    let material = MeshMaterial2d(materials.add(Color::from(GREEN_600)));
    let rect = Mesh2d(meshes.add(Rectangle::new(2.0 * PIPE_HALF_WIDTH, PIPE_HEIGHT)));
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

/// Spawn new pipes at the right edge of the screen
pub fn spawn_new_pipes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_pipes_at(
        MAX_WIDTH / 2.0 + PIPE_HALF_WIDTH,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
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

    if bottom < -MAX_HEIGHT / 2.0 || top > MAX_HEIGHT / 2.0 {
        exit.write(AppExit::Success);
    }
}

/// Move pipes and despawn a pipe if it goes out of bounds
pub fn move_pipes(
    pipes: Query<(&mut Transform, Entity), Or<(With<TopPipe>, With<BottomPipe>)>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut transform, entity) in pipes {
        let x = &mut transform.translation.x;
        *x -= PIPE_SPEED * time.delta_secs();
        if *x + PIPE_HALF_WIDTH < -MAX_WIDTH / 2.0 {
            // the pipe is out of bounds, delete it
            commands.entity(entity).despawn();
        }
    }
}

pub fn detect_collisions(
    player: Single<Entity, With<Player>>,
    pipes: Query<Entity, Or<(With<TopPipe>, With<BottomPipe>)>>,
    transform_helper: TransformHelper,
    mut exit: MessageWriter<AppExit>,
) {
    let transform = transform_helper.compute_global_transform(*player).unwrap();
    let player_collider = BoundingCircle::new(transform.translation().xy(), PLAYER_HALF_HEIGHT);

    for pipe in pipes {
        let transform = transform_helper.compute_global_transform(pipe).unwrap();
        let pipe_collider =
            Rectangle::new(PIPE_HEIGHT, PIPE_HEIGHT).aabb_2d(transform.translation().xy());

        if player_collider.intersects(&pipe_collider) {
            exit.write(AppExit::Success);
        }
    }
}
