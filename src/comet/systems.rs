use bevy::asset::AssetServer;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use super::components::*;

pub const COMET_SPEED: f32 = 50.0;

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

fn cursor_to_world_position(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_transform: &GlobalTransform,
    camera: &Camera,
) -> Option<Vec2> {
    let window = window_query.get_single().unwrap();

    if let Some(screen_pos) = window.cursor_position() {
        if let Some(world_position) = camera.viewport_to_world(camera_transform, screen_pos) {
            Some(world_position.origin.truncate())
        } else {
            None
        }
    } else {
        None
    }
}

pub fn click_to_explode(
    window_query: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    query_comets: Query<(Entity, &Transform), With<Comet>>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = query_camera.single();

    if let Some(world_cursor_pos) = cursor_to_world_position(window_query, camera_transform, camera) {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            for (entity, transform) in query_comets.iter() {
                let comet_pos = transform.translation.truncate();

                let distance = world_cursor_pos.distance(comet_pos);

                if distance < 50.0 {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
