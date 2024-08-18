mod level;
mod tilemap;
mod camera;
mod cable;

use bevy::prelude::*;
use bevy::window::EnabledButtons;
use crate::camera::setup_camera;
use crate::level::{test_layers};
use crate::tilemap::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "MAM DOSYC".into(),
                    resolution: ((MAP_WIDTH * TILE_SIZE) as f32, (MAP_HEIGHT * TILE_SIZE) as f32).into(),
                    enabled_buttons: EnabledButtons {
                        maximize: false,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            }),
        )
        .add_systems(Startup, (setup_camera, test_layers).chain())
        .run();
}
