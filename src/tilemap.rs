use std::collections::HashMap;
use bevy::prelude::*;

pub const MAP_WIDTH: i32 = 16;
pub const MAP_HEIGHT: i32 = 16;

pub const TILE_SIZE: i32 = 64;

pub enum TileType {
    EndEastCable = 1,
    EndWestCable = 3,
    EndSouthCable = 4,
    EndNorthCable = 12,
    HorizontalCable = 2,
    VerticalCable = 8,
    NorthWestCable = 10,
    NorthEastCable = 9,
    SouthWestCable = 6,
    SouthEastCable = 5,
}

#[derive(Resource)]
pub struct Tilemap {
    tiles: HashMap<IVec3, Entity>,
    atlas_layout: Handle<TextureAtlasLayout>,
    texture: Handle<Image>,
    offset: Vec2
}

impl Tilemap {
    pub fn new(atlas_layout: Handle<TextureAtlasLayout>, texture: Handle<Image>, offset: Vec2) -> Self {
        Self { atlas_layout, texture, tiles: HashMap::new(), offset }
    }

    pub fn grid_to_translation(&self, position: (i32, i32)) -> Vec2 {
        Vec2::new(
            (TILE_SIZE * position.0 + TILE_SIZE / 2) as f32 + self.offset.x,
            (TILE_SIZE * position.1 + TILE_SIZE / 2) as f32 + self.offset.y
        )
    }

    pub fn translation_to_grid(&self, translation: Vec2) -> (i32, i32) {
        let x = ((translation.x - self.offset.x) / TILE_SIZE as f32).floor() as i32;
        let y = ((translation.y - self.offset.y) / TILE_SIZE as f32).floor() as i32;

        (x, y)
    }

    pub fn set(&mut self, commands: &mut Commands, position: IVec3, tile_type: Option<TileType>) {
        if let Some(entity) = self.tiles.get(&position) {
            commands.entity(entity.clone()).despawn()
        }
        if tile_type.is_none() {
            self.tiles.remove(&position);
            return
        };

        let translation = Vec3::from(
            (self.grid_to_translation((position.x, position.y)), position.z as f32)
        );
        let entity = commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(translation),
                texture: self.texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(TILE_SIZE as f32)),
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: self.atlas_layout.clone(),
                index: tile_type.unwrap() as usize
            }
        )
        ).id();
        self.tiles.insert(position, entity);
    }
}

#[derive(Resource)]
pub struct TilemapFactory {
    pub(crate) atlas_layout: Handle<TextureAtlasLayout>,
    pub(crate) texture: Handle<Image>
}

impl TilemapFactory {
    pub fn instantiate(&self, offset: Vec2) -> Tilemap {
        Tilemap::new(self.atlas_layout.clone(), self.texture.clone(), offset)
    }
}