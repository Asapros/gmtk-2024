use bevy::prelude::*;
use crate::tilemap::{TileType, Tilemap};

pub fn random_path(seed: usize) -> Vec<(i32, i32)> {
    let paths = [
        vec![(-6, -2), (-5, -2), (-4, -2), (-3, -2), (-2, -2), (-1, -2), (-1, -1), (-1, 0), (-1, 1), (-1, 2), (-1, 3), (-1, 4), (0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4), (5, 3), (5, 2), (5, 1), (5, 0), (5, -1), (5, -2), (5, -3), (5, -4), (5, -5), (5, -6), (5, -7)],
        vec![(-6, 6), (-5, 6), (-4, 6), (-4, 5), (-4, 4), (-4, 3), (-4, 2), (-4, 1), (-4, 0), (-3, 0), (-2, 0), (-1, 0), (-1, -1), (-1, -2), (0, -2), (1, -2), (1, -1), (1, 0), (1, 1), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (6, 1), (6, 0), (7, 0)],
        vec![(-8, 1), (-7, 1), (-6, 1), (-5, 1), (-4, 1), (-3, 1), (-3, 0), (-3, -1), (-3, -2), (-3, -3), (-3, -4), (-2, -4), (-1, -4), (0, -4), (1, -4), (1, -3), (1, -2), (1, -1), (1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6), (5, 5), (5, 4), (5, 3), (5, 2), (5, 1), (6, 1), (7, 1)],
        vec![(-6, 7), (-6, 6), (-6, 5), (-6, 4), (-6, 3), (-6, 2), (-6, 1), (-5, 1), (-4, 1), (-3, 1), (-2, 1), (-2, 2), (-2, 3), (-2, 4), (-1, 4), (0, 4), (1, 4), (1, 3), (1, 2), (1, 1), (1, 0), (1, -1), (1, -2), (1, -3), (1, -4), (2, -4), (3, -4), (4, -4), (5, -4), (6, -4), (7, -4)]
    ];

    let mut seed = seed;
    for i in 1..100 {
        seed = ((seed + i) * 23) % 100000
    }
    seed += 5;
    // println!("{}", seed);


    let path = paths[(seed as f32 * 0.7).ceil() as usize % paths.len()].clone();
    let flip = (seed) % 2;
    let rotate = ((seed as f32 * 0.3).ceil() as usize) % 2;
    let flipped_path = if flip == 1 {
        path.iter().map(|coord| (coord.0 * -1, coord.1 * -1)).collect()
    } else {path};
    let rotated_path = if rotate == 1{
        flipped_path.iter().map(|coord| (coord.1, coord.0)).collect()
    } else {flipped_path};

    // println!("[DEBUG] {} {} {}", seed & paths.len(), flip, rotate);

    rotated_path
}

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
        (_, _) => {panic!("invalid cable path ({}, {}) -> ({}, {})", primary.0, primary.1, secondary.0, secondary.1)}
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