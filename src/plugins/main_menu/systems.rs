use crate::plugins::main_menu::layout::*;
use crate::plugins::main_menu::styles::*;
use crate::plugins::main_menu::*;
use bevy::app::AppExit;
use bevy::prelude::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu() {
    println!("DESPAWN");
}

#[allow(clippy::type_complexity)]
pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
) {
    // TODO: it would be nice to move all of this matching and querying to the common function.
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        interact_with_button(interaction, &mut background_color);
    }
}

#[allow(clippy::type_complexity)]
pub fn interact_with_prefs_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PrefsButton>),
    >,
) {
    // TODO: it would be nice to move all of this matching and querying to the common function.
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        interact_with_button(interaction, &mut background_color);
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
        interact_with_button(interaction, &mut background_color);
        if *interaction == Interaction::Pressed {
            exit_event_writer.send(AppExit)
        }
    }
}

pub fn interact_with_button(interaction: &Interaction, background_color: &mut BackgroundColor) {
    match *interaction {
        Interaction::Pressed => {
            *background_color = PRESSED_BUTTON_COLOR.into();
            //app_state_next_state.set(AppState::Game);
        }
        Interaction::Hovered => {
            *background_color = HOVERED_BUTTON_COLOR.into();
        }
        Interaction::None => {
            *background_color = NORMAL_BUTTON_COLOR.into();
        }
    };
}
