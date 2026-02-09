use bevy::{camera::ScalingMode, color::palettes::tailwind::RED_600, prelude::*};

const MAX_WIDTH: f32 = 640.0;
const MAX_HEIGHT: f32 = 360.0;

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
    let ellipse = Mesh2d(meshes.add(Ellipse::new(50.0, 25.0)));
    let material = MeshMaterial2d(materials.add(Color::from(RED_600)));

    commands.spawn((ellipse, material, Transform::from_xyz(0.0, 0.0, 1.0)));
}
