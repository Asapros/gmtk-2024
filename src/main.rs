mod level;
mod tilemap;
mod camera;

use bevy::prelude::*;
use crate::camera::setup_camera;
use crate::level::test_layers;
use crate::tilemap::setup_grid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_grid, setup_camera, test_layers).chain())
        .run();
}
