use bevy::prelude::*;
use std::f32::consts::PI;
use bevy::app::AppExit;
use bevy::asset::AssetServer;
use bevy::audio::AudioBundle;
use bevy::color::Color;
use bevy::core_pipeline::bloom::{BloomCompositeMode, BloomPrefilterSettings, BloomSettings};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::hierarchy::Children;
use bevy::input::ButtonInput;
use bevy::input::mouse::MouseWheel;
use bevy::math::{Vec2, Vec3};
use bevy::window::PrimaryWindow;
use rand::{random, Rng};

use crate::resources::*;
use crate::components::*;
use crate::constants::*;

pub fn spawn_planets(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    planets_res: Res<PlanetsCount>,
) {
    let window = window_query.get_single().unwrap();

    let mut rng = rand::thread_rng();
    let arm_count = 5;
    let arm_offset = 2.0 * PI / arm_count as f32;
    let planet_count: usize = planets_res.value;
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
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
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

pub fn zoom_scale(
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

pub fn setup_ui(
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
        parent.spawn(ButtonBundle {
            ..default()
        }).with_children(|button| {
            button.spawn(TextBundle {
                text: Text::from_section(
                    "+1000",
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 30.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ),
                ..default()
            });
        });

        parent.spawn(ButtonBundle {
            ..default()
        }).with_children(|button| {
            button.spawn(TextBundle {
                text: Text::from_section(
                    "+10000",
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 30.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ),
                ..default()
            });
        });
    });

    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..Default::default()
        },
        ..Default::default()
    })
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Planets: 1000 | FPS: 0.00",
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..Default::default()
                },
                StarFpsText,
            ));
        });
}

pub fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = query.single_mut();
    let speed = CAMERA_SPEED * time.delta_seconds();

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

pub fn setup_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/sound.ogg"),
        ..default()
    });
}

pub fn button_system(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
    mut planets_res: ResMut<PlanetsCount>,
) {
    for (interaction, children) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            for &child in children.iter() {
                if let Ok(text) = text_query.get_mut(child) {
                    if text.sections[0].value == "+1000" {
                        planets_res.value += 1000;
                    } else if text.sections[0].value == "+10000" {
                        planets_res.value += 10000;
                    }
                }
            }
        }
    }
}

pub fn update_planets(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    planets_res: Res<PlanetsCount>,
) {
    if planets_res.is_changed() && planets_res.value > 1000 {
        spawn_planets(commands, window_query, asset_server, planets_res);
    }
}

pub fn update_counts_ui(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<StarFpsText>>,
    planets_count: Res<PlanetsCount>,
) {
    let mut fps_text = "FPS: N/A".to_string();

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_value) = fps.average() {
            fps_text = format!("FPS: {:.2}", fps_value);
        }
    }

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Planets: {} | {}", planets_count.value, fps_text);
    }
}

pub fn exit_app(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit::Success);
    }
}