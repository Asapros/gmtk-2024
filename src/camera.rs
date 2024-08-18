use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    let color = Color::BLACK;
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(256.,0.,0.),
            camera: Camera {
                clear_color: ClearColorConfig::Custom(color),
                ..Default::default()
            },
            ..Default::default()
        }
    );
}