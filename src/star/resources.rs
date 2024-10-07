use bevy::prelude::*;

#[derive(Resource)]
pub struct StarsCount {
    pub value: usize,
}

impl Default for StarsCount {
    fn default() -> Self {
        StarsCount { value: 1000 }
    }
}
