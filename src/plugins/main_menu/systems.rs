use crate::plugins::main_menu::layout::*;
use crate::plugins::main_menu::*;
use crate::{plugins, AppState};
use bevy::app::AppExit;
use bevy::prelude::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

#[allow(clippy::type_complexity)]
pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    // TODO: it would be nice to move all of this matching and querying to the common function.
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        plugins::interact_with_button(interaction, &mut background_color, false);
        if *interaction == Interaction::Pressed {
            app_state_next_state.set(AppState::Play);
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn interact_with_prefs_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PrefsButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    // TODO: it would be nice to move all of this matching and querying to the common function.
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        plugins::interact_with_button(interaction, &mut background_color, false);
        if *interaction == Interaction::Pressed {
            app_state_next_state.set(AppState::PrefsDialog);
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut exit_event_writer: EventWriter<AppExit>,
) {
    // TODO: it would be nice to move all of this matching and querying to the common function.
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        plugins::interact_with_button(interaction, &mut background_color, false);
        if *interaction == Interaction::Pressed {
            exit_event_writer.send(AppExit)
        }
    }
}
