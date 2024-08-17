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
    texture: Handle<Image>
}

impl Tilemap {
    pub fn new(atlas_layout: Handle<TextureAtlasLayout>, texture: Handle<Image>) -> Self {
        Self { atlas_layout, texture, tiles: HashMap::new() }
    }

    pub fn set(&mut self, commands: &mut Commands, position: IVec3, tile_type: TileType) {
        if let Some(entity) = self.tiles.get(&position) {
            commands.entity(entity.clone()).despawn()
        }

        let entity = commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz((TILE_SIZE * position.x + TILE_SIZE / 2) as f32, (TILE_SIZE * position.y + TILE_SIZE / 2) as f32, position.z as f32),
                texture: self.texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(TILE_SIZE as f32)),
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: self.atlas_layout.clone(),
                index: tile_type as usize
            }
        )
        ).id();
        self.tiles.insert(position, entity);
    }
}

pub fn setup_grid(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>) {
    commands.insert_resource(Tilemap::new(
        texture_atlases.add(TextureAtlasLayout::from_grid(Vec2::splat(16.0), 4, 4, None, None)),
        assets.load("textures/rj45-tile.png")
    ));
}
