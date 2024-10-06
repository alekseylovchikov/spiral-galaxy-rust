use bevy::prelude::Resource;

#[derive(Resource)]
pub struct PlanetsCount {
    pub value: usize,
}

impl Default for PlanetsCount {
    fn default() -> Self {
        PlanetsCount { value: 1000 }
    }
}
