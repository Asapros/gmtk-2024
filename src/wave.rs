use bevy::prelude::*;
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

pub fn handle_continue_button(
    mut commands: Commands,
    mut wave_state_reader: EventReader<WaveStateChange>, mut manager: ResMut<LevelManager>, mut level_switch_reader: EventReader<LevelSwitchEvent>, state: Res<GameState>) {
    let mut level = manager.get_current_level_mut();
    for event in wave_state_reader.read() {
        if event.running {
            hide_continue_button(&mut commands, &mut level);
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