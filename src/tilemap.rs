use std::collections::HashMap;
use bevy::prelude::*;

// const MAP_WIDTH: u32 = 16;
// const MAP_HEIGHT: u32 = 16;

const TILE_SIZE: u32 = 32;

pub enum TileType {
    Other = 8,
    Orange = 7,
    Half = 10
}

#[derive(Resource)]
pub struct Tilemap {
    tiles: HashMap<UVec3, Entity>,
    atlas_layout: Handle<TextureAtlasLayout>,
    texture: Handle<Image>
}

impl Tilemap {
    pub fn new(atlas_layout: Handle<TextureAtlasLayout>, texture: Handle<Image>) -> Self {
        Self { atlas_layout, texture, tiles: HashMap::new() }
    }

    pub fn set(&mut self, commands: &mut Commands, position: UVec3, tile_type: TileType) {

        if let Some(entity) = self.tiles.get(&position) {
            commands.entity(entity.clone()).despawn()
        }

        let entity = commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz((TILE_SIZE * position.x) as f32, (TILE_SIZE * position.y) as f32, position.z as f32),
                texture: self.texture.clone(),
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
        texture_atlases.add(TextureAtlasLayout::from_grid(Vec2::splat(32.0), 6, 6, None, None)),
        assets.load("textures/tiles.png")
    ));
}
