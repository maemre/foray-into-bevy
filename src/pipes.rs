use bevy::{
    color::palettes::tailwind::GREEN_600,
    math::bounding::{Bounded2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};
use std::time::Duration;

use bevy::time::common_conditions::on_timer;

use super::*;

const PIPE_START: f32 = -MAX_WIDTH / 6.0;
const PIPE_END: f32 = MAX_WIDTH / 2.0 + PIPE_HALF_WIDTH;
const NUM_PIPES: usize = 3;
const PIPE_HALF_WIDTH: f32 = 50.0;
const PIPE_HEIGHT: f32 = MAX_HEIGHT - PIPE_GAP;
const PIPE_GAP: f32 = 150.0;
const PIPE_SPEED: f32 = 100.0;
pub const PIPE_SPAWN_PERIOD: f32 = MAX_WIDTH / (PIPE_SPEED * NUM_PIPES as f32);

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
    mut commands: Commands,
) {
    let transform = transform_helper.compute_global_transform(*player).unwrap();
    let player_collider = BoundingCircle::new(transform.translation().xy(), PLAYER_HALF_HEIGHT);

    for pipe in pipes {
        let transform = transform_helper.compute_global_transform(pipe).unwrap();
        let pipe_collider = Rectangle::new(PIPE_HALF_WIDTH * 2.0, PIPE_HEIGHT)
            .aabb_2d(transform.translation().xy());

        if player_collider.intersects(&pipe_collider) {
            commands.trigger(GameOver);
        }
    }
}

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pipes).add_systems(
            FixedUpdate,
            (
                gravity,
                check_out_of_bounds,
                move_pipes,
                spawn_new_pipes.run_if(on_timer(Duration::from_secs_f32(PIPE_SPAWN_PERIOD))),
                detect_collisions,
            ),
        );
    }
}
