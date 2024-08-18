use bevy::prelude::*;
use bevy::text::Text;

#[derive(Component)]
pub struct MoneyText;

pub const MENU_WIDTH: f32 = 256.0;

pub fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
           "MAM DOSYC",
           TextStyle {
               font: asset_server.load("fonts/QuinqueFive.ttf"),
               font_size: 40.0,
               color: Color::rgb(1., 0., 0.),
               ..default()
           },
        )
            .with_text_justify(JustifyText::Center)
            .with_style(Style{
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(10.0),
                ..default()
            }),
        MoneyText,
    ));
}
