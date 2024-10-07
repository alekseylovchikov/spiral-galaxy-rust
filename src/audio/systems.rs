use bevy::prelude::*;

pub fn setup_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/sound.ogg"),
        ..default()
    });
}