use bevy::prelude::*;

use crate::bug::{BugSprite};
use crate::level::LevelManager;
use crate::selection::{SelectionEvent, TileSelection};
use crate::tilemap::TileType;

#[derive(Resource)]
pub struct BugsAnimationTimer(pub Timer);

#[derive(Resource)]
pub struct SelectionAnimationTimer(pub Timer);

pub fn bugs_animation(mut timer: ResMut<BugsAnimationTimer>, mut query: Query<&mut TextureAtlas, With<BugSprite>>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut texture_atlas in query.iter_mut() {
            texture_atlas.index = (texture_atlas.index + 1) % 4;
        }
    }
}

pub fn config_selection_animation(
    mut commands: Commands,
    mut timer: ResMut<SelectionAnimationTimer>,
    mut selection_event_reader: EventReader<SelectionEvent>,
    tile_selection: Res<TileSelection>,
    mut manager: ResMut<LevelManager>,
    time: Res<Time>,
) {
    let mut level = manager.get_current_level_mut();
    for event in selection_event_reader.read() {
        // println!("[DEBUG] WOOOOOOOOOOOOOOOOOO");
        if let Some(selected) = event.selected {
            // println!("[DEBUG] s");
            level.tilemap.set(&mut commands, selected, Some(TileType::SelectionBigger));
        }
        if let Some(deselected) = event.deselected {
            // println!("[DEBUG] d");
            level.tilemap.set(&mut commands, deselected, None);
        }
        // timer.0.pause();
        // timer.0.reset();
        // timer.0.unpause();
    }
}

pub fn selection_animation(
    mut commands: Commands,
    mut timer: ResMut<SelectionAnimationTimer>,
    mut manager: ResMut<LevelManager>,
    mut selection_event_reader: EventReader<SelectionEvent>,
    time: Res<Time>,
) {
    let level = manager.get_current_level_mut();
    let tiles = [TileType::SelectionBigger, TileType::SelectionSmaller];
    if timer.0.tick(time.delta()).just_finished() {
        for tile in tiles {
            for event in selection_event_reader.read() {
                level.tilemap.set(&mut commands, event.selected.unwrap(), Some(tile));
            }
        }
    }
}