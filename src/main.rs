use bevy::{
    dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin},
    log::{Level, LogPlugin},
    prelude::*,
};
use foray_into_bevy::{pipes::PipePlugin, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            // filter: "bevy_dev_tools=trace,foray_into_bevy::pipes=info".into(),
            filter: "foray_into_bevy::pipes=info".into(),
            level: Level::ERROR,
            ..default()
        }))
        .add_plugins((MeshPickingPlugin, DebugPickingPlugin))
        .insert_resource(DebugPickingMode::Disabled)
        .add_plugins(PipePlugin)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (gravity, check_out_of_bounds))
        .add_systems(
            Update,
            (handle_input, update_score_text, toggle_pause, toggle_debug),
        )
        .run();
}
