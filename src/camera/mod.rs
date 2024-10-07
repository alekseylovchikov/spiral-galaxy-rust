use bevy::prelude::*;

mod systems;

use systems::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, zoom_scale)
            .add_systems(Update, camera_movement);
    }
}
