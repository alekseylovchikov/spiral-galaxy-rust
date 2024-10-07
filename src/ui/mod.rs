mod systems;
mod components;

use bevy::prelude::*;
use systems::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ui)
            .add_systems(Update, button_system)
            .add_systems(Update, update_counts_ui);
    }
}
