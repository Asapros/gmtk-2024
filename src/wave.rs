use bevy::log::tracing_subscriber::fmt::writer::EitherWriter::B;
use bevy::prelude::*;
use bevy::tasks::futures_lite::StreamExt;
use crate::bug::{BugFactory, BugSprite, BugType};
use crate::level::{Level, LevelManager};
use crate::selection::LevelSwitchEvent;
use crate::tilemap::TileType;
use crate::ui::{CONTINUE_COORDS, STEP_OUT_COORDS};

#[derive(Resource)]
pub struct GameState {
    pub health: u32,
    pub round_running: bool
}

#[derive(Event)]
pub struct WaveStateChange {
    pub running: bool
}

pub fn setup_game(mut commands: Commands, mut wave_state_writer: EventWriter<WaveStateChange>) {
    commands.insert_resource(GameState {health: 100, round_running: false});
    wave_state_writer.send(WaveStateChange{running: false});
}

pub fn show_continue_button(commands: &mut Commands, level: &mut Level) {
    level.tilemap.set(commands, IVec3::new(CONTINUE_COORDS[0].0, CONTINUE_COORDS[0].1, 10), Some(TileType::Continue1));
    level.tilemap.set(commands, IVec3::new(CONTINUE_COORDS[1].0, CONTINUE_COORDS[1].1, 10), Some(TileType::Continue2));
    level.tilemap.set(commands, IVec3::new(CONTINUE_COORDS[2].0, CONTINUE_COORDS[2].1, 10), Some(TileType::Continue3));
    level.tilemap.set(commands, IVec3::new(CONTINUE_COORDS[3].0, CONTINUE_COORDS[3].1, 10), Some(TileType::Continue4));
    if level.parent.is_some() {

        level.tilemap.set(commands, IVec3::new(STEP_OUT_COORDS[0].0, STEP_OUT_COORDS[0].1, 10), Some(TileType::StepOut1));
        level.tilemap.set(commands, IVec3::new(STEP_OUT_COORDS[1].0, STEP_OUT_COORDS[1].1, 10), Some(TileType::StepOut2));
        level.tilemap.set(commands, IVec3::new(STEP_OUT_COORDS[2].0, STEP_OUT_COORDS[2].1, 10), Some(TileType::StepOut3));
        level.tilemap.set(commands, IVec3::new(STEP_OUT_COORDS[3].0, STEP_OUT_COORDS[3].1, 10), Some(TileType::StepOut4));
    }
}


pub fn hide_continue_button(commands: &mut Commands, level: &mut Level) {
    level.tilemap.set(commands, IVec3::new(CONTINUE_COORDS[0].0, CONTINUE_COORDS[0].1, 10), None);
    level.tilemap.set(commands, IVec3::new(CONTINUE_COORDS[1].0, CONTINUE_COORDS[1].1, 10), None);
    level.tilemap.set(commands, IVec3::new(CONTINUE_COORDS[2].0, CONTINUE_COORDS[2].1, 10), None);
    level.tilemap.set(commands, IVec3::new(CONTINUE_COORDS[3].0, CONTINUE_COORDS[3].1, 10), None);

    level.tilemap.set(commands, IVec3::new(STEP_OUT_COORDS[0].0, STEP_OUT_COORDS[0].1, 10), None);
    level.tilemap.set(commands, IVec3::new(STEP_OUT_COORDS[1].0, STEP_OUT_COORDS[1].1, 10), None);
    level.tilemap.set(commands, IVec3::new(STEP_OUT_COORDS[2].0, STEP_OUT_COORDS[2].1, 10), None);
    level.tilemap.set(commands, IVec3::new(STEP_OUT_COORDS[3].0, STEP_OUT_COORDS[3].1, 10), None);
}

pub fn spawn_wave(mut commands: Commands, mut manager: ResMut<LevelManager>, bug_factory: Res<BugFactory>, mut state: ResMut<GameState>, mut wave_state_writer: EventWriter<WaveStateChange>) {
    if !state.round_running { return; }
    let mut level = manager.get_current_level_mut();
    level.bug_frames = (level.bug_frames + 1) % level.bug_queue_speed;
    if level.bug_frames != 0 { return; }

    let position = level.tilemap.grid_to_translation(level.cable[0]);
    let translation = Vec3::from((position, 2.0));
        // let translation = Vec3::from((0.0, 0.0, 2.0));
    let enemy = level.bug_queue.pop();

    match enemy {
        None => {
            // state.round_running = false;
            // wave_state_writer.send(WaveStateChange{running: false});

        }
        Some(BugType::Bug) => {
            let bug = bug_factory.instantiate_bugs(Transform::from_translation(translation));
            commands.spawn(bug);
        }
        _ => {}
    }

}

pub fn end_wave(bug_query: Query<(&BugSprite)>, mut state: ResMut<GameState>, mut wave_state_writer: EventWriter<WaveStateChange>, mut manager: ResMut<LevelManager>) {
    if !state.round_running { return; }
    let bugs = bug_query.iter().count();
    let mut level = manager.get_current_level_mut();
    if bugs > 0 || level.bug_queue.len() > 0 { return; }
    state.round_running = false;
    wave_state_writer.send(WaveStateChange{running: false});
    level.money += (500 * level.round) as i32;
    if level.parent.is_some() {
        let parent = level.parent.unwrap();
        let round = level.round;
        let active = manager.active;
        for (pos, mut tower) in manager.levels[parent].towers.iter_mut() {
            if tower.level_index != active { continue; }
            tower.upgrade_factor = round + 1;
        }
    }
}

pub fn handle_continue_button(
    mut commands: Commands,
    mut wave_state_reader: EventReader<WaveStateChange>, mut manager: ResMut<LevelManager>, mut level_switch_reader: EventReader<LevelSwitchEvent>, state: Res<GameState>) {
    let mut level = manager.get_current_level_mut();
    for event in wave_state_reader.read() {
        if event.running {
            level.round += 1;
            hide_continue_button(&mut commands, &mut level);
            let (queue, speed) = get_wave_composition(level.round);
            level.bug_queue = queue;
            level.bug_queue_speed = speed;
        } else {
            show_continue_button(&mut commands, &mut level);
        }
    }
    for event in level_switch_reader.read() {
        if state.round_running {
            hide_continue_button(&mut commands, &mut level);
        } else {
            show_continue_button(&mut commands, &mut level);
        }
    }
}

pub fn get_wave_composition(round: u32) -> (Vec<BugType>, u32) {
    match round {
        // 1 => (vec![BugType::Bug; 6], 180),
        1 => (vec![BugType::Hamster; 1], 1),
        2 => (vec![BugType::Bug; 20], 150),
        3 => (vec![BugType::Bug; 30], 100),
        4 => (vec![vec![BugType::Bug; 10], vec![BugType::Ant; 10], vec![BugType::Bug; 10]].into_iter().flatten().collect(), 140),
        5 => (vec![vec![BugType::Bug; 5], vec![BugType::Ant; 5], vec![BugType::Bug; 5], vec![BugType::Ant; 5]].into_iter().flatten().collect(), 80),
        6 => (vec![vec![BugType::Bug; 10], vec![BugType::Ant; 10], vec![BugType::Bug; 10], vec![BugType::Ant; 10]].into_iter().flatten().collect(), 80),
        7 => (vec![BugType::Ant; 30], 60),
        8 => (vec![vec![BugType::Bug; 5], vec![BugType::Ant; 5], vec![BugType::Bug; 5], vec![BugType::Ant; 5], vec![BugType::Bug; 10], vec![BugType::Ant; 10]].into_iter().flatten().collect(), 60),
        9 => (vec![vec![BugType::Bug; 5], vec![BugType::Ant; 5], vec![BugType::Bug; 5], vec![BugType::Ant; 5], vec![BugType::Bug; 10], vec![BugType::Ant; 10]].into_iter().flatten().collect(), 40),
        _ => (vec![BugType::Ant; 100], 10)
    }
}