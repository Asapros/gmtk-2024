use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;
use crate::level::{Level, LevelManager};
use crate::tilemap::{MAP_WIDTH, TILE_SIZE};
#[derive(Component)]
pub struct StatsText;

#[derive(Component)]
pub struct TowerStats;

pub const MENU_WIDTH: f32 = 368.0;

pub fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/QuinqueFive.ttf"),
                font_size: 30.0,
                color: Color::rgb(0., 1., 0.),
                ..default()
           },
        )
            .with_text_justify(JustifyText::Left)
            .with_style(Style{
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px((MAP_WIDTH * TILE_SIZE) as f32 + 20.0),
                ..default()
            }),
        StatsText,
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection {
                value: "".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/QuinqueFive.ttf"),
                    font_size: 15.0,
                    color: Color::rgb(0., 1., 0.),
                    ..default()
                },
            },
            TextSection {
                value: "".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/QuinqueFive.ttf"),
                    font_size: 7.5,
                    color: Color::rgb(0., 1., 0.),
                    ..default()
                },
            }
        ])
            .with_text_justify(JustifyText::Left)
            .with_style(Style{
                position_type: PositionType::Absolute,
                top: Val::Px(400.0),
                left: Val::Px((MAP_WIDTH * TILE_SIZE) as f32 + 20.0),
                ..default()
            }),
        TowerStats,
    ));

}

pub fn update_stats_text(mut text_query: Query<&mut Text, With<StatsText>>, manager: Res<LevelManager>) {
    let mut text = text_query.single_mut();
    let level = manager.get_current_level();
    text.sections[0].value = format!("Money: {}\nRound: {}", level.money.to_string(), "1");
}

pub fn debug_add_money(
    mut level_manager: ResMut<LevelManager>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut level = level_manager.get_current_level_mut();

    if keys.just_pressed(KeyCode::KeyO) {
        level.money += 10;
    }
    if keys.just_pressed(KeyCode::KeyP) {
        level.money -= 10;
    }
}