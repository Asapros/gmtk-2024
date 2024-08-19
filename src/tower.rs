use bevy::math::IVec3;
use bevy::prelude::*;
use crate::bug::BugSprite;
use crate::level::LevelManager;
use crate::selection::TowerBuildEvent;
use crate::tilemap::{TileType, Tilemap};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum TowerType {
    Resistor,
    Capacitor,
    Servo,
    Diode,
}

pub struct TowerSprite {
    tower_type: TowerType
}

pub const TOWER_TYPES: [TowerType; 4] = [TowerType::Resistor, TowerType::Capacitor, TowerType::Servo, TowerType::Diode];

pub fn tile_to_tower_types(tilemap: &Tilemap, position: (i32, i32)) -> Vec<TowerType> {
    if tilemap.is_occupied(IVec3::new(position.0, position.1, 3)) || tilemap.is_occupied(IVec3::new(position.0, position.1, 4)) {
        return vec![];
    }
    // println!("[DEBUG] {} {}", position.0, position.1);
    if tilemap.is_occupied(IVec3::new(position.0, position.1, 1)) {
        return vec![TowerType::Resistor];
    }
    vec![TowerType::Capacitor, TowerType::Diode, TowerType::Servo]
}

pub fn tower_type_to_tile(tower_type: &TowerType) -> (i32, i32){
    match tower_type {
        TowerType::Resistor => (11, 0),
        TowerType::Capacitor => (12, 0),
        TowerType::Servo => (11, 1),
        TowerType::Diode => (12, 1)
    }
}

pub fn tower_type_to_tile_type(tower_type: &TowerType) -> TileType {
    match tower_type {
        TowerType::Resistor => TileType::ResistorTower,
        TowerType::Capacitor => TileType::CapacitorTower,
        TowerType::Servo => TileType::Servo1,
        TowerType::Diode => TileType::LedOff,
    }
}

pub fn handle_build_tower(mut commands: Commands, mut tower_build_reader: EventReader<TowerBuildEvent>, mut manager: ResMut<LevelManager>) {
    let mut level = manager.get_current_level_mut();
    for event in tower_build_reader.read() {
        // println!("[DEBUG] build event");
        level.tilemap.set(&mut commands, IVec3::new(event.position.0, event.position.1, 4), Some(tower_type_to_tile_type(&event.tower)));
        level.towers.insert(event.position, TowerSprite {tower_type: event.tower});
    }
}


pub fn handle_resistor(mut bug_query: Query<(&Transform, &mut BugSprite)>, manager: Res<LevelManager>) {
    let level = manager.get_current_level();
    for (bug_transform, mut bug_sprite) in bug_query.iter_mut() {
        let tower_sprite = level.towers.get(&level.tilemap.translation_to_grid(Vec2::new(bug_transform.translation.x, bug_transform.translation.y)));
        bug_sprite.resistor_debuff = false;
        if tower_sprite.is_none() { continue; };
        if tower_sprite.unwrap().tower_type != TowerType::Resistor { continue };
        bug_sprite.resistor_debuff = true;
    }
}