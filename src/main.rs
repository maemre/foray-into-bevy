use bevy::prelude::*;
use foray_into_bevy::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_pipes))
        .add_systems(FixedUpdate, (gravity, check_out_of_bounds))
        .add_systems(Update, handle_input)
        .run();
}
