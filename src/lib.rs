use bevy::{color::palettes::tailwind::RED_600, prelude::*};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // spawn the camera, no change yet
    commands.spawn(Camera2d);

    // let's show a simple ellipse
    let ellipse = Mesh2d(meshes.add(Ellipse::new(50.0, 25.0)));
    let material = MeshMaterial2d(materials.add(Color::from(RED_600)));
    
    commands.spawn((
        ellipse,
        material,
        Transform::from_xyz(0.0, 0.0, 1.0),
        ));
}
