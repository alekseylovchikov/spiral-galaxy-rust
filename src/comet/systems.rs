use bevy::asset::AssetServer;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{default, Commands, Query, Res, SpriteBundle, Time, Transform, Window, With};
use bevy::window::PrimaryWindow;
use rand::random;

use super::components::*;

pub const COMET_SPEED: f32 = 200.0;

pub fn spawn_comets(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..3 {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0 - random_x, 0.0 - random_y, 0.0),
                texture: asset_server.load("sprites/fireball.png"),
                ..default()
            },
            Comet {
                direction: Vec2::new(random::<f32>(), random::<f32>()),
            }
        ));
    }
}

pub fn comet_movement(
    mut comet_query: Query<(&mut Transform, &Comet)>,
    time: Res<Time>,
) {
    for (mut transform, comet) in comet_query.iter_mut() {
        let direction = Vec3::new(comet.direction.x, comet.direction.y, 0.0);
        transform.translation += direction * COMET_SPEED * time.delta_seconds();
    }
}