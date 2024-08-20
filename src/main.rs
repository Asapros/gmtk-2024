mod level;
mod tilemap;
mod camera;
mod cable;
mod bug;
mod animations;
mod ui;
mod tower;
mod selection;
mod sounds;
mod wave;

use bevy::prelude::*;
use bevy::window::{EnabledButtons, PresentMode};
use crate::bug::{debug_spawn_bug, load_bugs, move_bugs, check_bug_health};
use crate::camera::setup_camera;
use crate::level::{setup_main_level, debug_level_switch, handle_level_switch};
use crate::tilemap::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};
use crate::animations::{BugsAnimationTimer, led_tower_animation, bugs_animation, SelectionAnimationTimer, config_selection_animation, selection_animation};
use crate::ui::{spawn_text, MENU_WIDTH, update_stats_text, debug_add_money, tower_options, tower_control_panel};
use crate::selection::{tile_selection, TileSelection, SelectionEvent, TowerBuildEvent, LevelSwitchEvent};
use crate::tower::{handle_build_tower, handle_resistor, handle_led, handle_capacitor, handle_capacitor_bullet};
use crate::sounds::{setup_sounds};
use crate::wave::{setup_game, WaveStateChange, handle_continue_button};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "STEP INTO".into(),
                    resolution: ((MAP_WIDTH * TILE_SIZE) as f32 + MENU_WIDTH, (MAP_HEIGHT * TILE_SIZE) as f32).into(),
                    resizable: false,
                    present_mode: PresentMode::AutoVsync,
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
        .add_event::<TowerBuildEvent>()
        .add_event::<LevelSwitchEvent>()
        .add_event::<WaveStateChange>()
        .add_systems(Startup, (setup_camera, setup_main_level, load_bugs, spawn_text, setup_sounds, setup_game))
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
            tower_options,
            handle_build_tower,
            handle_resistor,
            handle_led,
            check_bug_health,
            led_tower_animation,
            handle_capacitor,
            handle_capacitor_bullet,
            tower_control_panel,
            handle_level_switch.before(tower_options).before(tower_control_panel),
            handle_continue_button.after(tile_selection)
        ))
        .run();
}