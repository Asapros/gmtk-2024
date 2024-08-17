use bevy::prelude::*;

const MAP_WIDTH: usize = 16;
const MAP_HEIGHT: usize = 16;

const TILE_SIZE: usize = 32;

pub enum TileType {
    Grass,
    Water,
}

#[derive(Component)]
struct TileSprite {
    tile_type: TileType
}

#[derive(Resource)]
pub struct Tilemap {
    tiles: Vec<Entity>,
}

impl Tilemap {
    pub fn new(mut commands: &mut Commands, atlas_layout: Handle<TextureAtlasLayout>, texture: Handle<Image>) -> Self {
        let mut tiles = Vec::new();
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let id = commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz((TILE_SIZE * x) as f32, (TILE_SIZE * y) as f32, 0.0),
                        texture: texture.clone(),
                        ..default()
                    },
                    TextureAtlas {
                        layout: atlas_layout.clone(),
                        index: 0
                    }
                )
                ).id();
                tiles.push(id);
            }
        }
        Tilemap { tiles }
    }
}