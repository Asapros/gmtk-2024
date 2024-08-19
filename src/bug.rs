use std::os::windows::fs::FileTimesExt;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use crate::level::{Level, LevelManager};
use crate::cable::{delta, Direction};
use crate::tilemap::{TilemapFactory, TILE_SIZE};

const BUG_SPEED: f32 = 213.7;

#[derive(Component)]
pub struct BugSprite {
    cable_progress: usize
}

#[derive(Resource)]
pub struct BugFactory {
    pub(crate) texture: Handle<Image>,
    pub(crate) atlas_layout: Handle<TextureAtlasLayout>,
}


impl BugFactory {
    pub fn instantiate(&self, transform: Transform) -> (SpriteBundle, TextureAtlas, BugSprite){
        (
            SpriteBundle {
                texture: self.texture.clone(),
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
                cable_progress: 0
            }
        )
    }
}

pub fn load_bugs(mut commands: Commands, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>, assets: Res<AssetServer>) {
    let bug_factory = BugFactory {
        atlas_layout: texture_atlases.add(TextureAtlasLayout::from_grid(Vec2::splat(16.0), 4, 1, None, None)),
        texture: assets.load("sprites/bug01.png")
    };
    commands.insert_resource(bug_factory)
}

pub fn debug_spawn_bug(mut commands: Commands, mut manager: ResMut<LevelManager>, keys: Res<ButtonInput<KeyCode>>, bug_factory: Res<BugFactory>) {
    if keys.just_pressed(KeyCode::KeyJ) {
        let level = manager.get_current_level();
        let position = level.tilemap.grid_to_translation(level.cable[0]);
        let translation = Vec3::from((position, 2.0));
        // let translation = Vec3::from((0.0, 0.0, 2.0));
        let bug = bug_factory.instantiate(Transform::from_translation(translation));
        commands.spawn(bug);
    }
}

pub fn move_bugs(
    mut commands: Commands,
    mut bugs_query: Query<(Entity, &mut Transform, &mut BugSprite)>,
    manager: Res<LevelManager>,
    time: Res<Time>
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
                continue;
            }
            continue;
        }
        let direction_normalized = direction.normalize() * BUG_SPEED * time.delta_seconds();
        transform.translation += direction_normalized;
        let angle = direction_normalized.y.atan2(direction_normalized.x);
        // println!("[DEBUG] angle: {:?}", angle);
        transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
    }
}

