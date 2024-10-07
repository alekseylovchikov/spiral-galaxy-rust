use bevy::math::Vec2;
use bevy::prelude::Component;
use bevy::time::Timer;

#[derive(Component)]
pub struct Comet {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Explosion {
    pub timer: Timer,
}
