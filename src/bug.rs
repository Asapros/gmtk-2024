use std::time::{SystemTime, UNIX_EPOCH};
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use bevy::audio::*;
use crate::level::{Level, LevelManager};
use crate::cable::{delta, Direction};
use crate::wave::GameState;
use crate::sounds::{BugDeathSound};
use crate::tilemap::{TilemapFactory, TILE_SIZE};

const BUG_SPEED: f32 = 2.137;

#[derive(Component)]
pub struct BugSprite {
    pub cable_progress: usize,
    pub resistor_debuff: f32,
    pub health: i32,
    pub speed_factor: f32,
    pub hamster: bool
}

#[derive(Resource)]
pub struct BugFactory {
    pub(crate) texture: [Handle<Image>; 9],
    pub(crate) atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Clone)]
pub enum BugType {
    Bug,
    Ant,
    Hamster
}

impl BugFactory {
    pub fn instantiate_bugs(&self, transform: Transform) -> (SpriteBundle, TextureAtlas, BugSprite){
        let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let random = ((duration_since_epoch.as_nanos() * duration_since_epoch.as_nanos()) as usize) / 10;
        let seed = random % 4;
        let textures = self.texture.clone();
        (
            SpriteBundle {
                texture: textures[seed].clone(),
                transform,
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(TILE_SIZE as f32) * 0.8),
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: self.atlas_layout.clone(),
                index: 0
            },
            BugSprite {
                cable_progress: 0,
                resistor_debuff: 1.0,
                health: 1000,
                speed_factor: 1.0,
                hamster: false
            }
        )
    }

    pub fn instantiate_ants(&self, transform: Transform) -> (SpriteBundle, TextureAtlas, BugSprite){
        let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let random = ((duration_since_epoch.as_nanos() * duration_since_epoch.as_nanos()) as usize) / 10;
        let seed = (random % 4) + 4;
        let textures = self.texture.clone();
        (
            SpriteBundle {
                texture: textures[seed].clone(),
                transform,
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(TILE_SIZE as f32) * 0.8),
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: self.atlas_layout.clone(),
                index: 0
            },
            BugSprite {
                cable_progress: 0,
                resistor_debuff: 1.0,
                health: 500,
                speed_factor: 1.7,
                hamster: false
            }
        )
    }
    pub fn instantiate_hamster(&self, transform: Transform) -> (SpriteBundle, TextureAtlas, BugSprite){
        // let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        // let random = ((duration_since_epoch.as_nanos() * duration_since_epoch.as_nanos()) as usize) / 10;
        // let seed = (random % 4) + 4;
        let texture = self.texture[8].clone();
        (
            SpriteBundle {
                texture: texture.clone(),
                transform,
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(TILE_SIZE as f32) * 2.0),
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: self.atlas_layout.clone(),
                index: 0
            },
            BugSprite {
                cable_progress: 0,
                resistor_debuff: 1.0,
                health: 15000,
                speed_factor: 0.4,
                hamster: true
            }
        )
    }
}

pub fn load_bugs(mut commands: Commands, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>, assets: Res<AssetServer>) {
    let textures = [
        assets.load("sprites/Bug_sprite_01.png"),
        assets.load("sprites/Bug_sprite_02.png"),
        assets.load("sprites/Bug_sprite_03.png"),
        assets.load("sprites/Bug_sprite_04.png"),
        assets.load("sprites/Bug_sprite_05.png"),
        assets.load("sprites/Bug_sprite_06.png"),
        assets.load("sprites/Bug_sprite_07.png"),
        assets.load("sprites/Bug_sprite_08.png"),
        assets.load("sprites/Hamster-sprite-final.png"),
    ];
    let bug_factory = BugFactory {
        atlas_layout: texture_atlases.add(TextureAtlasLayout::from_grid(Vec2::splat(16.0), 4, 1, None, None)),
        texture: textures,
    };
    commands.insert_resource(bug_factory)
}

pub fn debug_spawn_bug(mut commands: Commands, mut manager: ResMut<LevelManager>, keys: Res<ButtonInput<KeyCode>>, bug_factory: Res<BugFactory>) {
    if keys.just_pressed(KeyCode::KeyJ) {
        let level = manager.get_current_level();
        let position = level.tilemap.grid_to_translation(level.cable[0]);
        let translation = Vec3::from((position, 2.0));
        // let translation = Vec3::from((0.0, 0.0, 2.0));
        let bug = bug_factory.instantiate_ants(Transform::from_translation(translation));
        commands.spawn(bug);
    }
}

pub fn move_bugs(
    mut commands: Commands,
    mut bugs_query: Query<(Entity, &mut Transform, &mut BugSprite)>,
    manager: Res<LevelManager>,
    mut state: ResMut<GameState>
) {
    let level = manager.get_current_level();
    for (entity, mut transform, mut bug_sprite) in bugs_query.iter_mut() {
        let focus_tile = level.cable[bug_sprite.cable_progress];
        let focus_coordinates = level.tilemap.grid_to_translation(focus_tile);
        let direction = Vec3::from((focus_coordinates, transform.translation.z)) - transform.translation;
        if direction.length() < 3.0 {
            bug_sprite.cable_progress += 1;
            if bug_sprite.cable_progress >= level.cable.len() {
                commands.entity(entity).despawn();
                state.health -= 10;
                if bug_sprite.hamster {
                    state.health -= 2137;
                }
                continue;
            }
            continue;
        }
        let direction_normalized = direction.normalize()
            * BUG_SPEED
            // * time.delta_seconds()
            * bug_sprite.resistor_debuff * bug_sprite.speed_factor;
        transform.translation += direction_normalized;
        let angle = direction_normalized.y.atan2(direction_normalized.x);
        // println!("[DEBUG] angle: {:?}", angle);
        transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
    }
}

pub fn check_bug_health(
    mut commands: Commands,
    mut bugs_query: Query<(Entity, &mut BugSprite)>,
    asset_server: Res<AssetServer>
) {
    for (entity, mut bug_sprite) in bugs_query.iter_mut() {
        if bug_sprite.health <= 0 {
            // println!("[DEBUG] Despawning bug: {:?}", entity);
            commands.spawn((
                AudioBundle {
                    source: asset_server.load("sounds/bug_die.ogg"),
                    settings: PlaybackSettings {
                        paused: false,
                        mode: PlaybackMode::Despawn,
                        volume: Volume::new(0.05),
                        ..default()
                    },
                    ..default()
                },
            ));
            commands.entity(entity).despawn();
        }
    }
}