use std::fmt::format;
use bevy::prelude::*;
use crate::cable::set_cable;
use crate::tilemap::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE, Tilemap, TileType};

pub enum LevelTheme {
    Black ,
    Blue,
    Green,
    Red
}

fn get_background(theme: &LevelTheme, asset_server: &Res<AssetServer>) -> Handle<Image> {
    let filename = match theme {
        LevelTheme::Black => "black",
        LevelTheme::Blue => "blue",
        LevelTheme::Green => "green",
        LevelTheme::Red => "red"
    };
    asset_server.load(format!("backgrounds/{}.png", filename))
}

pub struct Level {
    offset: Vec2,
    tilemap: Tilemap,
    cable: Vec<(i32, i32)>,
    theme: LevelTheme
}

impl Level {
    pub fn setup(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands.spawn( (
            SpriteBundle {
                texture: get_background(&self.theme, asset_server),
                transform: Transform::from_xyz(self.offset.x, self.offset.y, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new((MAP_WIDTH * TILE_SIZE) as f32, (MAP_HEIGHT * TILE_SIZE) as f32)),
                    ..default()
                },
                ..default()
            },
        ));
        set_cable(&mut self.tilemap, commands, &self.cable)
    }
}

pub struct LevelManager {
    levels: Vec<Level>
}


pub fn test_layers(mut commands: Commands, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>, assets: Res<AssetServer>) {
    let path = vec![
        (-6, 2), (-5, 2), (-4, 2), (-3, 2), (-2, 2), (-1, 2), (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (6, 3), (5, 3), (4, 3), (3, 3), (2, 3), (1, 3), (0, 3)
    ];
    let tilemap = Tilemap::new(
        texture_atlases.add(TextureAtlasLayout::from_grid(Vec2::splat(16.0), 4, 4, None, None)),
        assets.load("textures/rj45-tile.png")
    );
    let mut level = Level {
        offset: Vec2::splat(0.0),
        tilemap,
        cable: path,
        theme: LevelTheme::Black,
    };
    level.setup(&mut commands, &assets)
}