use crate::plugins::despawn_entity;
use crate::plugins::play::systems::*;
use crate::plugins::play::PlayScreen;
use crate::AppState;
use bevy::prelude::*;

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Play), spawn_play_screen)
            .add_systems(OnExit(AppState::Play), despawn_entity::<PlayScreen>);
    }
}
