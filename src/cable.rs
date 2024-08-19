use bevy::prelude::*;
use crate::tilemap::{TileType, Tilemap};

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    West,
    East
}

pub fn delta(primary: &(i32, i32), secondary: &(i32, i32)) -> Direction {
    let difference = (primary.0 - secondary.0, primary.1 - secondary.1);
    match difference {
        (0, -1) => {Direction::North},
        (1, 0) => {Direction::West},
        (0, 1) => {Direction::South},
        (-1, 0) => {Direction::East},
        (_, _) => {panic!("invalid cable path")}
    }
}

pub fn set_cable(tilemap: &mut Tilemap, commands: &mut Commands, path: &Vec<(i32, i32)>) {
    for (index, coordinate) in path.iter().enumerate() {
        let previous = if index == 0 {None} else {path.get(index-1)};
        let next = path.get(index+1);

        let previous_delta = if previous.is_none() {None} else {Some(delta(coordinate, previous.unwrap()))};
        let next_delta = if next.is_none() {None} else {Some(delta(coordinate, next.unwrap()))};

        let tile_type = match (previous_delta, next_delta) {
            (None, Some(Direction::East)) => TileType::EndEastCable,
            (Some(Direction::East), None) => TileType::EndEastCable,
            (None, Some(Direction::West)) => TileType::EndWestCable,
            (Some(Direction::West), None) => TileType::EndWestCable,
            (None, Some(Direction::South)) => TileType::EndSouthCable,
            (Some(Direction::South), None) => TileType::EndSouthCable,
            (None, Some(Direction::North)) => TileType::EndNorthCable,
            (Some(Direction::North), None) => TileType::EndNorthCable,
            (Some(Direction::West), Some(Direction::East)) => TileType::HorizontalCable,
            (Some(Direction::East), Some(Direction::West)) => TileType::HorizontalCable,
            (Some(Direction::South), Some(Direction::North)) => TileType::VerticalCable,
            (Some(Direction::North), Some(Direction::South)) => TileType::VerticalCable,
            (Some(Direction::North), Some(Direction::East)) => TileType::NorthEastCable,
            (Some(Direction::East), Some(Direction::North)) => TileType::NorthEastCable,
            (Some(Direction::North), Some(Direction::West)) => TileType::NorthWestCable,
            (Some(Direction::West), Some(Direction::North)) => TileType::NorthWestCable,
            (Some(Direction::South), Some(Direction::East)) => TileType::SouthEastCable,
            (Some(Direction::East), Some(Direction::South)) => TileType::SouthEastCable,
            (Some(Direction::West), Some(Direction::South)) => TileType::SouthWestCable,
            (Some(Direction::South), Some(Direction::West)) => TileType::SouthWestCable,
            (_, _) => {unreachable!()}
        };
        let z= match tile_type {
            TileType::EndNorthCable => 3,
            TileType::EndSouthCable => 3,
            TileType::EndWestCable => 3,
            TileType::EndEastCable => 3,
            _ => 1
        };
        tilemap.set(commands, IVec3::new(coordinate.0, coordinate.1, z), Some(tile_type))
    }
}