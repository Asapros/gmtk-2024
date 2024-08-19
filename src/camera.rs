use bevy::prelude::*;

pub const CAMERA_OFFSET: f32 = 200.0;
pub fn setup_camera(mut commands: Commands) {
    let color = Color::BLACK;
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(CAMERA_OFFSET,0.,0.),
            camera: Camera {
                clear_color: ClearColorConfig::Custom(color),
                ..Default::default()
            },
            ..Default::default()
        }
    );
}