use bevy::prelude::*;
use bevy::audio::*;

#[derive(Component)]
pub struct BugDeathSound;

#[derive(Component)]
pub struct BackgroundMusic;

pub fn setup_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn((
    //     AudioBundle {
    //         source: asset_server.load("sounds/bug_die.ogg"),
    //         settings: PlaybackSettings {
    //             paused: false,
    //             mode: PlaybackMode::Despawn,
    //             volume: Volume::new(1.0),
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     BugDeathSound{}
    // ));

    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/music.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(0.2),
                ..default()
            },
            ..default()
        },
        BackgroundMusic,
    ));
}