use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::asset::AssetServer;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::window::PrimaryWindow;

pub const FONT_PATH: &str = "fonts/FiraSans-Bold.ttf";

pub fn spawn_black_hole(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        texture: asset_server.load("sprites/Black_hole.png"),
        ..default()
    });
}

pub fn exit_app(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit::Success);
    }
}