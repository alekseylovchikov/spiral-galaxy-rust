use bevy::prelude::*;

mod systems;

use systems::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_audio);
    }
}
