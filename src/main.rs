use bevy::prelude::*;
use foray_into_bevy::{pipes::PipePlugin, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PipePlugin)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                gravity,
                check_out_of_bounds,
            ),
        )
        .add_systems(Update, (handle_input, display_score))
        .run();
}
