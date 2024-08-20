use bevy::audio::{PlaybackMode, Volume};
use bevy::math::IVec3;
use bevy::prelude::*;
use crate::bug::BugSprite;
use crate::cable::random_path;
use crate::level::{LevelManager, LevelTheme, TilemapFactoryResource};
use crate::selection::TowerBuildEvent;
use crate::tilemap::{TileType, Tilemap, TilemapFactory, TILE_SIZE};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum TowerType {
    Resistor,
    Capacitor,
    // Servo,
    Diode,
}

pub struct TowerSprite {
    pub tower_type: TowerType,
    pub frame_counter: u32,
    pub upgrade_factor: u32,
    pub balance: i32,
    pub level_index: usize
}

pub const TOWER_TYPES: [TowerType; 3] = [TowerType::Resistor, TowerType::Capacitor, TowerType::Diode];

pub const DIODE_FRAMES: u32 = 60;
pub const CAPACITOR_FRAMES: u32 = 240;
pub fn tile_to_tower_types(tilemap: &Tilemap, position: (i32, i32)) -> Vec<TowerType> {
    if tilemap.is_occupied(IVec3::new(position.0, position.1, 3)) || tilemap.is_occupied(IVec3::new(position.0, position.1, 4)) {
        return vec![];
    }
    // println!("[DEBUG] {} {}", position.0, position.1);
    if tilemap.is_occupied(IVec3::new(position.0, position.1, 1)) {
        return vec![TowerType::Resistor];
    }
    vec![TowerType::Capacitor, TowerType::Diode]
}

pub fn tower_type_to_tile(tower_type: &TowerType) -> (i32, i32){
    match tower_type {
        TowerType::Resistor => (11, -1),
        TowerType::Capacitor => (12, 0),
        // TowerType::Servo => (12, 1),
        TowerType::Diode => (11, 0)
    }
}

pub fn tower_type_to_tile_type(tower_type: &TowerType) -> TileType {
    match tower_type {
        TowerType::Resistor => TileType::ResistorTower,
        TowerType::Capacitor => TileType::CapacitorTower,
        // TowerType::Servo => TileType::Servo1,
        TowerType::Diode => TileType::LedOff,
    }
}

// pub fn spawn_tower_control_panel(mut commands: Co)
pub fn handle_build_tower(
    mut commands: Commands,
    mut tower_build_reader: EventReader<TowerBuildEvent>,
    mut manager: ResMut<LevelManager>,
    tilemap_factory: Res<TilemapFactoryResource>,
    asset_server: Res<AssetServer>,
    time: Res<Time>
) {
    for event in tower_build_reader.read() {
        let parent = Some(manager.active.clone());
        let random = (time.elapsed_seconds() * 100.0) as usize;
        let recursed = manager.add_level(LevelTheme::Blue, random_path(random), &tilemap_factory.0, &mut commands, &asset_server, parent);
        let mut level = manager.get_current_level_mut();
        // println!("[DEBUG] build event");
        level.tilemap.set(&mut commands, IVec3::new(event.position.0, event.position.1, 4), Some(tower_type_to_tile_type(&event.tower)));
        level.towers.insert(event.position, TowerSprite {tower_type: event.tower, frame_counter: 0, upgrade_factor: 1, balance: 0, level_index: recursed});
        commands.spawn((
            AudioBundle {
                source: asset_server.load("sounds/place.ogg"),
                settings: PlaybackSettings {
                    paused: false,
                    mode: PlaybackMode::Despawn,
                    volume: Volume::new(0.1),
                    ..default()
                },
                ..default()
            },
        ));
    }
}


pub fn handle_resistor(mut bug_query: Query<(&Transform, &mut BugSprite)>, manager: Res<LevelManager>) {
    let level = manager.get_current_level();
    for (bug_transform, mut bug_sprite) in bug_query.iter_mut() {
        let tower_sprite = level.towers.get(&level.tilemap.translation_to_grid(Vec2::new(bug_transform.translation.x, bug_transform.translation.y)));
        bug_sprite.resistor_debuff = 1.0;
        if tower_sprite.is_none() { continue; };
        if tower_sprite.unwrap().tower_type != TowerType::Resistor { continue };
        bug_sprite.resistor_debuff = 0.5 / tower_sprite.unwrap().upgrade_factor as f32;
    }
}

pub fn handle_led(mut bug_query: Query<(&Transform, &mut BugSprite)>, mut manager: ResMut<LevelManager>) {
    let mut level = manager.get_current_level_mut();
    for (position, mut tower) in level.towers.iter_mut() {
        if tower.tower_type != TowerType::Diode { continue };
        tower.frame_counter = (tower.frame_counter + 1) % DIODE_FRAMES;

        let hurt = tower.frame_counter == 0;
        if hurt {
            // println!("[DEBUG] hurt");
            for (bug_transform, mut bug_sprite) in bug_query.iter_mut() {
                if level.tilemap.grid_to_translation(position.clone()).distance(Vec2::new(bug_transform.translation.x, bug_transform.translation.y)) > (TILE_SIZE as f32) * 1.5 {
                    continue;
                }
                bug_sprite.health -= 250;
                bug_sprite.health -= (tower.upgrade_factor * 50) as i32;
            }
        }
    }
}

#[derive(Component)]
pub struct CapacitorTarget {
    id: f32
}

#[derive(Component)]
pub struct CapacitorBullet {
    id: f32,
    damage: i32
}

pub fn handle_capacitor(
    mut commands: Commands,
    mut bug_query: Query<(&Transform, Entity, &mut BugSprite)>,
    mut manager: ResMut<LevelManager>,
    time: Res<Time>,
    asset_server: Res<AssetServer>
) {
    let mut level = manager.get_current_level_mut();
    for (position, mut tower) in level.towers.iter_mut() {
        if tower.tower_type != TowerType::Capacitor { continue };
        tower.frame_counter = (tower.frame_counter + 1) % CAPACITOR_FRAMES;

        let shoot = tower.frame_counter == 0;
        if shoot {
            for (bug_transform, entity, mut bug_sprite) in bug_query.iter_mut() {
                if level.tilemap.grid_to_translation(position.clone()).distance(Vec2::new(bug_transform.translation.x, bug_transform.translation.y)) > (TILE_SIZE as f32) * 6.0 {
                    continue;
                }
                let target = commands.get_entity(entity);
                if target.is_none() { continue; };
                let id = time.elapsed_seconds();
                // println!("[DEBUG] bullet id: {:?}", id);
                target.unwrap().insert(CapacitorTarget{id});
                commands.spawn((SpriteBundle {
                    texture: asset_server.load("sprites/bullet.png"),
                    transform: Transform::from_translation(Vec3::from((level.tilemap.grid_to_translation(position.clone()), 5.0))),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(TILE_SIZE as f32 / 2.0)),
                        ..default()
                    },
                    ..default()
                }, CapacitorBullet{id, damage: 750 + (100 * tower.upgrade_factor) as i32}
                ));
                break;
            }
        }
    }
}
const BULLET_SPEED: f32 = 21.37;
pub fn handle_capacitor_bullet(mut commands: Commands, mut bullet_query: Query<(Entity, &mut Transform, &CapacitorBullet), Without<CapacitorTarget>>, mut target_query: Query<(&Transform, &mut BugSprite, &CapacitorTarget)>) {
    for (bullet_entity, mut bullet_transform, bullet_meta) in bullet_query.iter_mut() {
        let mut found = false;
        // println!("[DEBUG] bul. {:?}", bullet_meta.id);
        for (target_transform, mut target_sprite, target_meta) in target_query.iter_mut() {
            // println!("[DEBUG] tg. {:?}", target_meta.id);
            if bullet_meta.id != target_meta.id { continue };
            found = true;
            if bullet_transform.translation.truncate().distance(target_transform.translation.truncate()) < TILE_SIZE as f32 * 0.2 {
                // println!("[DEBUG] hit");
                commands.entity(bullet_entity).despawn();
                target_sprite.health -= bullet_meta.damage;
                break;
            }
            let direction = target_transform.translation.truncate() - bullet_transform.translation.truncate();
            let direction_normalized = direction.normalize() * BULLET_SPEED;
            bullet_transform.translation += Vec3::from((direction_normalized, 0.0));
            break;
        }
        if !found {
            commands.entity(bullet_entity).despawn();
        }
    }
}