pub mod constants;
pub mod components;
mod systems;
pub mod star;
pub mod comet;

use systems::*;
use star::*;

use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default()),
        )
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins(StarPlugin)
        .add_systems(Startup, setup_audio)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, spawn_black_hole)
        .add_systems(Update, zoom_scale)
        .add_systems(Update, camera_movement)
        .add_systems(Update, button_system)
        .add_systems(Update, update_counts_ui)
        .add_systems(Update, exit_app)
        .run();
}
