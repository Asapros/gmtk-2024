mod level;
mod tilemap;
mod camera;
mod cable;
mod bug;
mod animations;
mod ui;
mod tower;
mod selection;

use bevy::prelude::*;
use bevy::window::EnabledButtons;
use crate::bug::{debug_spawn_bug, load_bugs, move_bugs};
use crate::camera::setup_camera;
use crate::level::{setup_main_level, debug_level_switch};
use crate::tilemap::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};
use crate::animations::{BugsAnimationTimer, bugs_animation, SelectionAnimationTimer, config_selection_animation, selection_animation};
use crate::ui::{spawn_text, MENU_WIDTH, update_stats_text, debug_add_money};
use crate::selection::{tile_selection, TileSelection, SelectionEvent, tower_options};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "MAM DOSYC".into(),
                    resolution: ((MAP_WIDTH * TILE_SIZE) as f32 + MENU_WIDTH, (MAP_HEIGHT * TILE_SIZE) as f32).into(),
                    resizable: false,
                    // mode: bevy::window::WindowMode::SizedFullscreen,
                    enabled_buttons: EnabledButtons {
                        maximize: false,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            }),
        )
        .insert_resource(TileSelection{tile: None})
        .insert_resource(BugsAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert_resource(SelectionAnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .add_event::<SelectionEvent>()
        .add_systems(Startup, (setup_camera, setup_main_level, load_bugs, spawn_text))
        .add_systems(Update, (
            debug_level_switch,
            debug_spawn_bug,
            move_bugs,
            bugs_animation,
            config_selection_animation,
            // selection_animation,
            update_stats_text,
            tile_selection,
            debug_add_money,
            tower_options
        ))
        .run();
}