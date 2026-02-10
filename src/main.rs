use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use foray_into_bevy::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_pipes))
        .add_systems(
            FixedUpdate,
            (
                gravity,
                check_out_of_bounds,
                move_pipes,
                spawn_new_pipes.run_if(on_timer(Duration::from_secs_f32(PIPE_SPAWN_PERIOD))),
                detect_collisions,
            ),
        )
        .add_systems(Update, handle_input)
        .run();
}
