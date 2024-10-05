use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;
use rand::Rng;
use bevy::core_pipeline::bloom::{BloomCompositeMode, BloomPrefilterSettings, BloomSettings};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::EventReader;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_buttons)
        .add_systems(Startup, spawn_planets)
        .add_systems(Startup, spawn_black_hole)
        .add_systems(Update, zoom_scale)
        .add_systems(Update, camera_movement)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Planet {}

#[derive(Component)]
pub struct BlackHole {}

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

pub fn spawn_black_hole(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/Black_hole.png"),
                ..default()
            },
            BlackHole {},
        )
    );
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        BloomSettings {
            intensity: 0.65,
            low_frequency_boost: 0.9,
            low_frequency_boost_curvature: 0.15,
            high_pass_frequency: 1.0,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 0.0,
                threshold_softness: 0.0,
            },
            composite_mode: BloomCompositeMode::Additive,
            ..default()
        }
    ));
}

fn zoom_scale(
    mut query_camera: Query<&mut OrthographicProjection, With<Camera>>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    let mut projection = query_camera.single_mut();

    for ev in evr_scroll.read() {
        if ev.y > 0.0 {
            projection.scale *= 1.25;
        } else {
            projection.scale /= 1.25;

        }
    }
}

fn spawn_buttons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            ..default()
        },
        background_color: Color::WHITE.into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "+1000",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            ..default()
        });

        parent.spawn(TextBundle {
            text: Text::from_section(
                "+10000",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            ..default()
        });
    });
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = query.single_mut();
    let speed = 500.0 * time.delta_seconds();

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        transform.translation.y += speed;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= speed;
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= speed;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += speed;
    }
}
