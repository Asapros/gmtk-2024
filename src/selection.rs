use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::tilemap::TileType;
use crate::level::LevelManager;

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
        println!("[DEBUG] select: {:?} deselect: {:?}", event.selected, event.deselected);
        if event.selected.is_none() {
            level.tilemap.set(&mut commands, IVec3::new(11, 0, 2), None);
            level.tilemap.set(&mut commands, IVec3::new(12, 0, 2), None);
            continue;
        }
        level.tilemap.set(&mut commands, IVec3::new(11, 0, 2), Some(TileType::HorizontalCable));
        level.tilemap.set(&mut commands, IVec3::new(12, 0, 2), Some(TileType::HorizontalCable));
    }
}
pub fn tile_selection(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut manager: ResMut<LevelManager>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut tile_selection: ResMut<TileSelection>,
    mut selection_event_writer: EventWriter<SelectionEvent>
) {
    // println!("[DEBUG] tile selection: {:?}", tile_selection.tile);
    let (camera, camera_transform) = camera_query.single();

    let window = window_query.single();

    let world_position = window.cursor_position().and_then(|cursor| camera.viewport_to_world(camera_transform, cursor)).map(|ray| ray.origin.truncate());
    if world_position.is_none() { return };
    let mut level = manager.get_current_level_mut();
    let hovered = level.tilemap.translation_to_grid(world_position.unwrap());
    let tile = IVec3::new(hovered.0, hovered.1, 5);
    if buttons.just_pressed(MouseButton::Left) {
        let previous = tile_selection.tile;
        if tile.x > 7 {
            if previous.is_none() {
                return;
            }
            tile_selection.tile = None;
            selection_event_writer.send(SelectionEvent{deselected: previous, selected: tile_selection.tile});

            if tile.x == 11 && tile.y == 0 {
                println!("[Debug] left")
            }
            if tile.x == 12 && tile.y == 0 {
                println!("[DEBUG] right")
            }
            return;
        }
        // if tile.x < -7 { return; }

        if (tile_selection.tile == Some(tile)) {
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