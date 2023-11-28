use crate::plugins::despawn_entity;
use crate::plugins::prefs::systems::*;
use crate::plugins::prefs::PrefsDialog;
use crate::AppState;
use bevy::prelude::*;

pub struct PrefsPlugin;

impl Plugin for PrefsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::PrefsDialog), spawn_prefs_dialog)
            .add_systems(
                Update,
                interact_with_checkbox.run_if(in_state(AppState::PrefsDialog)),
            )
            // .add_systems(
            //     Update,
            //     (
            //         //interact_with_play_button,
            //         //interact_with_prefs_button,
            //         //interact_with_quit_button,
            //     )
            //         .run_if(in_state(AppState::MainMenu)),
            // )
            .add_systems(OnExit(AppState::PrefsDialog), despawn_entity::<PrefsDialog>);
    }
}
