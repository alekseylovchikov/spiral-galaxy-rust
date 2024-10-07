use std::f32::consts::PI;
use bevy::asset::AssetServer;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use super::components::*;
use super::resources::*;

pub fn update_stars(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    stars_res: Res<StarsCount>,
) {
    if stars_res.is_changed() && stars_res.value > 1000 {
        spawn_stars(commands, window_query, asset_server, stars_res);
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    stars_res: Res<StarsCount>,
) {
    let window = window_query.get_single().unwrap();

    let mut rng = rand::thread_rng();
    let arm_count = 5;
    let arm_offset = 2.0 * PI / arm_count as f32;
    let stars_count: usize = stars_res.value;
    let offset: usize = 100;

    let mut stars_positions = Vec::new();

    for i in 0..stars_count {
        let arm = (i + offset) % arm_count;
        let arm_angle = arm as f32 * arm_offset;

        let t = (i as f32 / stars_count as f32).sqrt();
        let theta = t * 10.0 * PI;
        let radius = 40.0 + 100.0 * t + i as f32;

        let angle_noise = rng.gen_range(-0.05..0.05);
        let radius_noise = rng.gen_range(-1.0..1.0);

        let x = (radius + radius_noise) * (arm_angle + theta + angle_noise).cos();
        let y = (radius + radius_noise) * (arm_angle + theta + angle_noise).sin();

        let mut valid_position = true;
        for &(existing_x , existing_y) in &stars_positions {
            let distance = ((x as f32 - existing_x as f32).powi(2) + (y as f32 - existing_y as f32).powi(2)).sqrt();
            if distance < 5.0 {
                valid_position = false;
                break;
            }
        }

        if valid_position {
            stars_positions.push((x, y));
            let star_size: f32 = rng.gen_range(0.2..5.9);
            let random_star_index: u8 = rng.gen_range(1..5);
            let planet_asset_path = match random_star_index {
                1 => "sprites/Baren.png",
                2 => "sprites/Ice.png",
                3 => "sprites/Lava.png",
                4 => "sprites/Terran.png",
                _ => "sprites/Terran.png"
            };

            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(window.width() / 2.0 + x, window.height() / 2.0 + y, 0.0),
                        scale: Vec3::splat(star_size),
                        ..default()
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(star_size)),
                        ..default()
                    },
                    texture: asset_server.load(planet_asset_path),
                    ..default()
                },
                Star {},
            ));
        }
    }
}
