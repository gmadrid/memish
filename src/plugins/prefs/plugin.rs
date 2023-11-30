use crate::plugins::despawn_entity;
use crate::plugins::prefs::systems::*;
use crate::plugins::prefs::{PrefSetterEvent, PrefsDialog};
use crate::AppState;
use bevy::prelude::*;

pub struct PrefsPlugin;

impl Plugin for PrefsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::PrefsDialog), spawn_prefs_dialog)
            .add_systems(OnExit(AppState::PrefsDialog), despawn_entity::<PrefsDialog>)
            .add_systems(
                Update,
                interact_with_basic_button.run_if(in_state(AppState::PrefsDialog)),
            )
            .add_systems(
                Update,
                read_pref_setter_events
                    .run_if(in_state(AppState::PrefsDialog))
                    .after(interact_with_basic_button),
            )
            .add_systems(
                Update,
                recompute_selected.run_if(in_state(AppState::PrefsDialog)),
            )
            .add_systems(
                Update,
                update_checkbox_display
                    .run_if(in_state(AppState::PrefsDialog))
                    .after(interact_with_basic_button)
                    .after(recompute_selected),
            )
            .add_event::<PrefSetterEvent>();
    }
}
