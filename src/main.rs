mod level;
mod tilemap;
mod camera;

use bevy::prelude::*;
use crate::camera::setup_camera;
use crate::level::setup_grid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_grid, setup_camera))
        .run();
}
