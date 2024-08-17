use bevy::prelude::*;
use crate::cable::set_cable;
use crate::tilemap::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE, Tilemap, TileType};
pub fn test_layers(mut commands: Commands, mut tilemap: ResMut<Tilemap>) {
    set_cable(&mut tilemap, &mut commands, vec![
        (-6, 2), (-5, 2), (-4, 2), (-3, 2), (-2, 2), (-1, 2), (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (6, 3), (5, 3), (4, 3), (3, 3), (2, 3), (1, 3), (0, 3)
    ]);
}

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn( (
        SpriteBundle {
            texture: asset_server.load("backgrounds/green.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new((MAP_WIDTH * TILE_SIZE) as f32, (MAP_HEIGHT * TILE_SIZE) as f32)),
                ..default()
            },
            ..default()
        },
    ));
}