use bevy::prelude::*;
use crate::tilemap::{Tilemap, TileType};
pub fn test_layers(mut commands: Commands, mut tilemap: ResMut<Tilemap>) {
    tilemap.set(&mut commands, UVec3::new(3, 3, 5), TileType::Orange);
    tilemap.set(&mut commands, UVec3::new(3, 3, 5), TileType::Half)
}