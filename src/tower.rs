use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::level::{Level, LevelManager};
use crate::tilemap::TileType;

#[derive(Resource)]
pub struct TileSelection {
    pub(crate) tile: Option<IVec3>
}

pub fn tile_selection(
        mut commands: Commands,
        window_query: Query<&Window, With<PrimaryWindow>>,
        camera_query: Query<(&Camera, &GlobalTransform)>,
        mut manager: ResMut<LevelManager>,
        buttons: Res<ButtonInput<MouseButton>>,
        mut tile_selection: ResMut<TileSelection>
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
        if let Some(selected) = tile_selection.tile {
            level.tilemap.set(&mut commands, selected, None)
        }
        if (tile_selection.tile == Some(tile)) {
            tile_selection.tile = None;
            return;
        }

        tile_selection.tile = Some(tile);
        // println!("[DEBUG] cursor position: {:?}", tile);
        level.tilemap.set(&mut commands, tile, Some(TileType::EndNorthCable))
    }
}
