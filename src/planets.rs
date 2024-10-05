use std::f32::consts::PI;
use bevy::asset::AssetServer;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{default, Commands, Query, Res, Sprite, SpriteBundle, Transform, Window, With};
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::Planet;

pub fn spawn_planets(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let mut rng = rand::thread_rng();
    let arm_count = 5;
    let arm_offset = 2.0 * PI / arm_count as f32;
    let planet_count: usize = 10000;
    let offset: usize = 100;

    let mut planets_positions = Vec::new();

    for i in 0..planet_count {
        let arm = (i + offset) % arm_count;
        let arm_angle = arm as f32 * arm_offset;

        let t = (i as f32 / planet_count as f32).sqrt();
        let theta = t * 10.0 * PI;
        let radius = 40.0 + 100.0 * t + i as f32;

        let angle_noise = rng.gen_range(-0.05..0.05);
        let radius_noise = rng.gen_range(-1.0..1.0);

        let x = (radius + radius_noise) * (arm_angle + theta + angle_noise).cos();
        let y = (radius + radius_noise) * (arm_angle + theta + angle_noise).sin();

        let mut valid_position = true;
        for &(existing_x , existing_y) in &planets_positions {
            let distance = ((x as f32 - existing_x as f32).powi(2) + (y as f32 - existing_y as f32).powi(2)).sqrt();
            if distance < 5.0 {
                valid_position = false;
                break;
            }
        }

        if valid_position {
            planets_positions.push((x, y));
            let planet_size: f32 = rng.gen_range(0.2..5.9);
            let random_planet_index: u8 = rng.gen_range(1..5);
            // let random_planet_path = format!("sprites/planet_{random_planet_index}.png");
            let planet_asset_path = match random_planet_index {
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
                        scale: Vec3::splat(planet_size),
                        ..default()
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(planet_size)),
                        ..default()
                    },
                    texture: asset_server.load(planet_asset_path),
                    ..default()
                },
                Planet {},
            ));
        }
    }
}
