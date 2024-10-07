use bevy::prelude::*;

pub mod systems;
pub mod components;
pub mod resources;

use resources::*;
use systems::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<StarsCount>()
            .add_systems(Startup, spawn_stars)
            .add_systems(Update, update_stars);
    }
}
