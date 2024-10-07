use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component)]
pub struct BlackHole {}

#[derive(Component)]
pub struct Comet {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct StarFpsText;
