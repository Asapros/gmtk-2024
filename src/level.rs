use bevy::prelude::*;
use crate::tilemap::Tilemap;

pub fn setup_grid(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>) {
    let texture: Handle<Image> = assets.load("textures/tiles.png");
    let atlas = TextureAtlasLayout::from_grid(Vec2::splat(32.0), 6, 6, None, None);
    let texture_atlas_handle = texture_atlases.add(atlas);
    let tilemap = Tilemap::new(&mut commands, texture_atlas_handle, texture);
}