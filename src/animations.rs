use bevy::prelude::*;

use crate::bug::{BugSprite};

#[derive(Resource)]
pub struct AnimationTimer(pub Timer);

pub fn bugs_animation(mut timer: ResMut<AnimationTimer>, mut query: Query<&mut TextureAtlas, With<BugSprite>>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut texture_atlas in query.iter_mut() {
            texture_atlas.index = (texture_atlas.index + 1) % 4;
        }
    }
}
