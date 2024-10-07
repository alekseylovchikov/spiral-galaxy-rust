use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::hierarchy::Children;
use bevy::prelude::{default, Button, ButtonBundle, Changed, Commands, Interaction, NodeBundle, PositionType, Query, Res, ResMut, Style, Text, TextBundle, TextStyle, Val, With};

use crate::star::resources::StarsCount;

use super::components::*;

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
                        font: asset_server.load(crate::systems::FONT_PATH),
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
                        font: asset_server.load(crate::systems::FONT_PATH),
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
                            font: asset_server.load(crate::systems::FONT_PATH),
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