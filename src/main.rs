use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use foray_into_bevy::{pipes::PipePlugin, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "foray_into_bevy::pipes=info".into(),
            level: Level::ERROR,
            ..default()
        }))
        .add_plugins(PipePlugin)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (gravity, check_out_of_bounds))
        .add_systems(Update, (handle_input, update_score_text, toggle_pause))
        .run();
}
