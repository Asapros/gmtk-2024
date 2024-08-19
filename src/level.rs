use std::collections::HashMap;
use std::fmt::format;
use bevy::prelude::*;
use crate::cable::set_cable;
use crate::tilemap::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE, Tilemap, TileType, TilemapFactory};
use crate::camera::CAMERA_OFFSET;
use crate::tower::{TowerType, TowerSprite};

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
    pub(crate) tilemap: Tilemap,
    pub(crate) cable: Vec<(i32, i32)>,
    theme: LevelTheme,
    pub money: i32,
    pub towers: HashMap<(i32, i32), TowerSprite>
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
#[derive(Resource)]
pub struct LevelManager {
    levels: Vec<Level>,
    active: usize
}

impl LevelManager {
    pub fn switch_view(&mut self, index: usize, transform: &mut Transform) {
        self.active = index;
        let level = &self.levels[index];
        transform.translation = Vec3::new(level.offset.x + CAMERA_OFFSET, level.offset.y, 0.0);
    }

    pub fn add_level(&mut self, theme: LevelTheme, cable: Vec<(i32, i32)>, tilemap_factory: &TilemapFactory, commands: &mut Commands, asset_server: &Res<AssetServer>, money: i32) -> usize {
        let offset = Vec2::new((self.levels.len() * 2000) as f32, 0.0);
        let mut level = Level {
            offset,
            tilemap: tilemap_factory.instantiate(offset),
            cable,
            theme,
            money,
            towers: HashMap::new()
        };
        level.setup(commands, asset_server);
        self.levels.push(level);

        self.levels.len() - 1
    }
    pub fn get_current_level(&self) -> &Level {
        &self.levels[self.active]
    }
    pub fn get_current_level_mut(&mut self) -> &mut Level {
        &mut self.levels[self.active]
    }
}


pub fn setup_main_level(mut commands: Commands, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>, assets: Res<AssetServer>) {
    let tilemap_factory = TilemapFactory {
        atlas_layout: texture_atlases.add(TextureAtlasLayout::from_grid(Vec2::splat(16.0), 8, 4, None, None)),
        texture: assets.load("tiles/TileSet.png")
    };

    let path = vec![
        (-6, 2), (-5, 2), (-4, 2), (-3, 2), (-2, 2), (-1, 2), (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (6, 3), (5, 3), (4, 3), (3, 3), (2, 3), (1, 3), (0, 3)
    ];
    let path2 = vec![
        (5, 5), (4, 5), (4, 6), (3, 6), (3, 7), (2, 7), (2, 6), (1, 6), (0, 6), (0, 5), (-1, 5), (-2, 5), (-2, 4), (-3, 4), (-3, 3), (-4, 3), (-4, 2), (-5, 2), (-5, 1), (-6, 1)
    ];
    let mut manager = LevelManager {levels: vec![], active: 0};
    manager.add_level(LevelTheme::Green, path.clone(), &tilemap_factory, &mut commands, &assets, 69);
    manager.add_level(LevelTheme::Black, path2.clone(), &tilemap_factory, &mut commands, &assets, 420);

    commands.insert_resource(manager);
    commands.insert_resource(tilemap_factory);
}

pub fn debug_level_switch(mut manager: ResMut<LevelManager>, keys: Res<ButtonInput<KeyCode>>, mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_position = camera_query.single_mut();

    if keys.just_pressed(KeyCode::KeyH) {
        let new_index = (manager.active + 1) % manager.levels.len();
        println!("[DEBUG] switching to {}", manager.active + 1 % manager.levels.len());
        manager.switch_view(new_index, &mut camera_position)
    }
}