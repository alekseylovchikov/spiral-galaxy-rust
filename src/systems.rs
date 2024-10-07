use bevy::prelude::*;
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
use rand::*;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;

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
                        "Stars: 1000 | FPS: 0.00",
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
    mut stars_res: ResMut<StarsCount>,
) {
    for (interaction, children) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            for &child in children.iter() {
                if let Ok(text) = text_query.get_mut(child) {
                    if text.sections[0].value == "+1000" {
                        stars_res.value += 1000;
                    } else if text.sections[0].value == "+10000" {
                        stars_res.value += 10000;
                    }
                }
            }
        }
    }
}

pub fn update_counts_ui(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<StarFpsText>>,
    stars_count: Res<StarsCount>,
) {
    let mut fps_text = "FPS: N/A".to_string();

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_value) = fps.average() {
            fps_text = format!("FPS: {:.2}", fps_value);
        }
    }

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Stars: {} | {}", stars_count.value, fps_text);
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