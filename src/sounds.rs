use bevy::prelude::*;
use bevy::audio::*;

#[derive(Component)]
pub struct BugDieSound;

#[derive(Component)]
pub struct BackgroundMusic;

pub fn setup_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/bug_die.ogg"),
            settings: PlaybackSettings {
                paused: true,
                mode: PlaybackMode::Once,
                volume: Volume::new(1.0),
                ..default()
            },
            ..default()
        },
        BugDieSound,
    ));

    // commands.spawn((
    //     AudioBundle {
    //         source: asset_server.load("sounds/music.ogg"),
    //         settings: PlaybackSettings {
    //             mode: PlaybackMode::Loop,
    //             volume: 0.8,
    //         }
    //         ..default()
    //     },
    //     BackgroundMusic,
    // ));
}