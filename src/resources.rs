use bevy::prelude::Resource;

#[derive(Resource)]
pub struct StarsCount {
    pub value: usize,
}

impl Default for StarsCount {
    fn default() -> Self {
        StarsCount { value: 1000 }
    }
}
