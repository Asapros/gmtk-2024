use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;
use crate::wave::GameState;
use crate::level::{Level, LevelManager};
use crate::selection::SelectionEvent;
use crate::tilemap::{TileType, MAP_WIDTH, TILE_SIZE};
use crate::tower::{tile_to_tower_types, tower_type_to_tile, tower_type_to_tile_type, TOWER_TYPES, TowerSprite};
#[derive(Component)]
pub struct TowerInfo;

#[derive(Component)]
pub struct StatsText;

#[derive(Component)]
pub struct TowerStatistics;

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
                top: Val::Px(500.0),
                left: Val::Px((MAP_WIDTH * TILE_SIZE) as f32 + 20.0),
                ..default()
            }),
        TowerInfo{},
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection {
                value: "".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/QuinqueFive.ttf"),
                    font_size: 20.0,
                    color: Color::GREEN,
                    ..default()
                }
            }
        ]).with_text_justify(JustifyText::Left).with_style(
            Style {
                position_type: PositionType::Absolute,
                top: Val::Px(300.0),
                left: Val::Px((MAP_WIDTH * TILE_SIZE) as f32 + 20.0),
                ..default()
            }
        ), TowerStatistics{}
    ));

}

pub fn update_stats_text(mut text_query: Query<&mut Text, With<StatsText>>, manager: Res<LevelManager>, state: Res<GameState>) {
    let mut text = text_query.single_mut();
    let level = manager.get_current_level();
    text.sections[0].value = if level.parent.is_none() {
        format!("Bit$: {}\nRnd:  {}/20\nHP:   {}", level.money.to_string(), level.round.to_string(), state.health)
    } else {
        format!("Bit$: {}\nRnd:  {}", level.money.to_string(), level.round.to_string())
    }
    // let cap = if level.parent.is_none() { "/20" } else {""};
    // text.sections[0].value = format!("Bits: {}\nRnd:  {}{}\nHealth: {}", level.money.to_string(), level.round.to_string(), cap, state.health);
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

pub fn tower_options(mut commands: Commands, mut selection_event_reader: EventReader<SelectionEvent>, mut manager: ResMut<LevelManager>) {
    let mut level = manager.get_current_level_mut();
    for event in selection_event_reader.read() {
        // println!("[DEBUG] select: {:?} deselect: {:?}", event.selected, event.deselected);
        for tower_type in TOWER_TYPES {
            let tower_tile = tower_type_to_tile(&tower_type);
            level.tilemap.set(&mut commands, IVec3::new(tower_tile.0, tower_tile.1, 0), None);
        }
        if event.selected.is_none() {
            continue;
        }
        for tower_type in tile_to_tower_types(&level.tilemap, (event.selected.unwrap().x, event.selected.unwrap().y)) {
            let tower_tile = tower_type_to_tile(&tower_type);
            level.tilemap.set(&mut commands, IVec3::new(tower_tile.0, tower_tile.1, 0), Some(tower_type_to_tile_type(&tower_type)));
        }
    }
}
fn show_control_panel(mut level: &mut Level, commands: &mut Commands, text: &mut Mut<Text>, tile_position: &(i32, i32), running: bool) {
    level.tilemap.set(commands, IVec3::new(DELETE_COORDS[0].0, DELETE_COORDS[0].1, 10), Some(TileType::Delete1));
    level.tilemap.set(commands, IVec3::new(DELETE_COORDS[1].0, DELETE_COORDS[1].1, 10), Some(TileType::Delete2));
    level.tilemap.set(commands, IVec3::new(DELETE_COORDS[2].0, DELETE_COORDS[2].1, 10), Some(TileType::Delete3));
    level.tilemap.set(commands, IVec3::new(DELETE_COORDS[3].0, DELETE_COORDS[3].1, 10), Some(TileType::Delete4));

    if !running {
        level.tilemap.set(commands, IVec3::new(RECURSE_COORDS[0].0, RECURSE_COORDS[0].1, 10), Some(TileType::StepInto1));
        level.tilemap.set(commands, IVec3::new(RECURSE_COORDS[1].0, RECURSE_COORDS[1].1, 10), Some(TileType::StepInto2));
        level.tilemap.set(commands, IVec3::new(RECURSE_COORDS[2].0, RECURSE_COORDS[2].1, 10), Some(TileType::StepInto3));
        level.tilemap.set(commands, IVec3::new(RECURSE_COORDS[3].0, RECURSE_COORDS[3].1, 10), Some(TileType::StepInto4));

    }

    level.tilemap.set(commands, IVec3::new(DONATE_COORDS[0].0, DONATE_COORDS[0].1, 10), Some(TileType::Donate1));
    level.tilemap.set(commands, IVec3::new(DONATE_COORDS[1].0, DONATE_COORDS[1].1, 10), Some(TileType::Donate2));
    level.tilemap.set(commands, IVec3::new(DONATE_COORDS[2].0, DONATE_COORDS[2].1, 10), Some(TileType::Donate3));
    level.tilemap.set(commands, IVec3::new(DONATE_COORDS[3].0, DONATE_COORDS[3].1, 10), Some(TileType::Donate4));


    let tower = level.towers.get(tile_position).unwrap();
    text.sections[0].value = format!("Tower stats:\n\nBit$:    {}\nUpgrade: {}", tower.balance, tower.upgrade_factor).to_string();
}
fn hide_control_panel(mut level: &mut Level, commands: &mut Commands, text: &mut Mut<Text>) {
    for delete_segment in DELETE_COORDS.iter().chain(DONATE_COORDS.iter()).chain(RECURSE_COORDS.iter()) {
        level.tilemap.set(commands, IVec3::new(delete_segment.0, delete_segment.1, 10), None);
    }

    text.sections[0].value = "".to_string();
}
pub const DELETE_COORDS: [(i32, i32); 4] = [(10, -7), (11, -7), (12, -7), (13, -7)];
pub const RECURSE_COORDS: [(i32, i32); 4] = [(10, -5), (11, -5), (12, -5), (13, -5)];
pub const DONATE_COORDS: [(i32, i32); 4] = [(10, -3), (11, -3), (12, -3), (13, -3)];
pub const STEP_OUT_COORDS: [(i32, i32); 4] = [(10, 2), (11, 2), (12, 2), (13, 2)];
pub const CONTINUE_COORDS: [(i32, i32); 4] = [(10, 4), (11, 4), (12, 4), (13, 4)];


pub fn tower_control_panel(
    mut commands: Commands,
    mut selection_event_reader: EventReader<SelectionEvent>,
    mut manager: ResMut<LevelManager>,
    mut tower_stats_query: Query<&mut Text, With<TowerStatistics>>,
    state: Res<GameState>
) {
    let mut tower_stats = tower_stats_query.get_single_mut().unwrap();

    let mut level = manager.get_current_level_mut();
    for event in selection_event_reader.read() {
        // println!("level {:?}", level.money);
        if event.selected.is_none() {
            hide_control_panel(&mut level, &mut commands, &mut tower_stats);
            continue;
        }
        let tile_position = (event.selected.unwrap().x, event.selected.unwrap().y);
        if level.towers.get(&tile_position).is_some() {
            show_control_panel(&mut level, &mut commands, &mut tower_stats, &tile_position, state.round_running);
        } else {
            hide_control_panel(&mut level, &mut commands, &mut tower_stats);
            continue;
        }
    }
}