use crate::plugins::play::layout::*;
use crate::prefs::Prefs;
use bevy::prelude::*;

pub fn spawn_play_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    prefs: Res<Prefs>,
) {
    build_play_screen(&mut commands, &asset_server, &prefs)
}
