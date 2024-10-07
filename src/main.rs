
mod systems;
pub mod star;
pub mod comet;
mod camera;
mod audio;
mod ui;

use systems::*;
use star::*;
use camera::*;
use comet::*;
use audio::*;
use ui::*;

use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default()),
        )
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0))) // Background color
        .add_plugins(CameraPlugin)
        .add_plugins(StarPlugin)
        .add_plugins(CometPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(UiPlugin)
        .add_systems(Startup, spawn_black_hole)
        .add_systems(Update, exit_app)
        .run();
}
