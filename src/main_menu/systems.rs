use crate::main_menu::*;
use bevy::app::AppExit;
use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

fn main_menu_style() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        height: Val::Percent(100.0),
        width: Val::Percent(100.0),
        row_gap: Val::Px(8.0),
        column_gap: Val::Px(8.0),
        ..Style::DEFAULT
    }
}

fn button_style() -> Style {
    Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        height: Val::Px(80.0),
        width: Val::Px(200.0),
        ..Style::DEFAULT
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: main_menu_style(),
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            layout_button_child(parent, "Play", PlayButton, asset_server);
            layout_button_child(parent, "Prefs", PrefsButton, asset_server);
            layout_button_child(parent, "Quit", QuitButton, asset_server);
        })
        .id()
}

fn layout_button_child(
    parent: &mut ChildBuilder,
    label: &str,
    component: impl Component,
    asset_server: &Res<AssetServer>,
) {
    parent
        // BUTTON
        .spawn((
            ButtonBundle {
                style: button_style(),
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            component, //TheButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(label, get_button_text_style(asset_server))],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        });
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
