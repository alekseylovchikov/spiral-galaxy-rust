
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
    let mut app = App::default();

    app
        .add_plugins((
             DefaultPlugins
                 .set(WindowPlugin {
                     primary_window: Some(Window {
                         title: "Galaxy Application".to_string(),
                         ..default()
                     }),
                     ..default()
                 }),
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default()),
        )
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins(CameraPlugin)
        .add_plugins(StarPlugin)
        .add_plugins(CometPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(UiPlugin)
        .add_systems(Startup, spawn_black_hole)
        .add_systems(Update, exit_app)
        .run();
}
