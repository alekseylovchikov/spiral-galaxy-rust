use bevy::prelude::*;

mod systems;
mod components;

use systems::*;

pub struct CometPlugin;

impl Plugin for CometPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_comets)
            .add_systems(Update, comet_movement)
            .add_systems(Update, click_to_explode)
            .add_systems(Update, explosion_cleanup);
    }
}
