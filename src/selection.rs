use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::tilemap::TileType;
use crate::level::LevelManager;
use crate::tower::{tile_to_tower_types, tower_type_to_tile, tower_type_to_tile_type, TowerType, TOWER_TYPES};
use crate::ui::TowerStats;

#[derive(Resource)]
pub struct TileSelection {
    pub(crate) tile: Option<IVec3>
}

#[derive(Event)]
pub struct SelectionEvent {
    pub(crate) deselected: Option<IVec3>,
    pub(crate) selected: Option<IVec3>
}

pub fn tower_options(mut commands: Commands, mut selection_event_reader: EventReader<SelectionEvent>, mut manager: ResMut<LevelManager>) {
    let mut level = manager.get_current_level_mut();
    for event in selection_event_reader.read() {
        // println!("[DEBUG] select: {:?} deselect: {:?}", event.selected, event.deselected);
        for tower_type in TOWER_TYPES {
            let tower_tile = tower_type_to_tile(&tower_type);
            level.tilemap.set(&mut commands, IVec3::new(tower_tile.0, tower_tile.1, 0), None);
        }
        if event.selected.is_none() {
            continue;
        }
        for tower_type in tile_to_tower_types(&level.tilemap, (event.selected.unwrap().x, event.selected.unwrap().y)) {
            let tower_tile = tower_type_to_tile(&tower_type);
            level.tilemap.set(&mut commands, IVec3::new(tower_tile.0, tower_tile.1, 0), Some(tower_type_to_tile_type(&tower_type)));
        }
    }
}

#[derive(Event)]
pub struct TowerBuildEvent {
    pub(crate) tower: TowerType,
    pub(crate) position: (i32, i32)
}
pub fn tile_selection(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut manager: ResMut<LevelManager>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut tile_selection: ResMut<TileSelection>,
    mut selection_event_writer: EventWriter<SelectionEvent>,
    mut tower_build_event_writer: EventWriter<TowerBuildEvent>,
    mut text_query: Query<&mut Text, With<TowerStats>>
) {
    // println!("[DEBUG] tile selection: {:?}", tile_selection.tile);
    let (camera, camera_transform) = camera_query.single();

    let window = window_query.single();

    let world_position = window.cursor_position().and_then(|cursor| camera.viewport_to_world(camera_transform, cursor)).map(|ray| ray.origin.truncate());
    if world_position.is_none() { return };
    let mut level = manager.get_current_level_mut();
    let hovered = level.tilemap.translation_to_grid(world_position.unwrap());
    let tile = IVec3::new(hovered.0, hovered.1, 5);

    let mut text = text_query.single_mut();
    text.sections[0].value = "".to_string();
    text.sections[1].value = "".to_string();
    if tile_selection.tile.is_some() {
        for tower_type in tile_to_tower_types(&level.tilemap, (tile_selection.tile.unwrap().x, tile_selection.tile.unwrap().y)) {
            let tower_option_tile = tower_type_to_tile(&tower_type);
            if hovered == tower_option_tile {
                match tower_type {
                    TowerType::Resistor => text.sections[0].value = "Slows bugs down\n\nRange:  Very low\nDamage: No\nSpeed:  Indefinite".to_string(),
                    TowerType::Capacitor => text.sections[0].value = "Zaps bugs\n\nRange:  High\nDamage: High\nSpeed:  Low".to_string(),
                    TowerType::Servo => {
                        text.sections[0].value = "Suffocates bugs\n\nRange:  Line\nDamage: Low\nSpeed:  Indefinite\n\nNote:\n   no rotation!\n   (left only)".to_string();
                        text.sections[1].value = "\n\n    (this is definitely a feature)".to_string();
                    },
                    TowerType::Diode => text.sections[0].value = "Flashes bugs\n\nRange:  Low\nDamage: Medium\nSpeed:  Fast".to_string(),
                }
            }
        }
    }

    if buttons.just_pressed(MouseButton::Left) {
        let previous = tile_selection.tile;
        if tile.x > 7 {
            if previous.is_none() {
                return;
            }
            tile_selection.tile = None;
            selection_event_writer.send(SelectionEvent{deselected: previous, selected: tile_selection.tile});

            for tower_type in tile_to_tower_types(&level.tilemap, (previous.unwrap().x, previous.unwrap().y)) {
                let tower_option_tile = tower_type_to_tile(&tower_type);
                // println!("[DEBUG] tt: {:?}", tower_type);
                if hovered == tower_option_tile {
                    // println!("[DEBUG] pass");
                    tower_build_event_writer.send(TowerBuildEvent{tower: tower_type, position: (previous.unwrap().x, previous.unwrap().y)});
                    break;
                }
            }
            return;
        }
        // if tile.x < -7 { return; }

        if tile_selection.tile == Some(tile) {
            tile_selection.tile = None;
            selection_event_writer.send(SelectionEvent{deselected: previous, selected: tile_selection.tile});
            return;
        }

        tile_selection.tile = Some(tile);
        selection_event_writer.send(SelectionEvent{deselected: previous, selected: tile_selection.tile});
        // println!("[DEBUG] cursor position: {:?}", tile);
        // level.tilemap.set(&mut commands, tile, Some(TileType::EndNorthCable))
    }
}