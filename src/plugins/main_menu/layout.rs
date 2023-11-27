use crate::plugins::main_menu::styles::{
    button_style, get_button_text_style, main_menu_style, title_text_style, NORMAL_BUTTON_COLOR,
};
use crate::plugins::main_menu::{MainMenu, PlayButton, PrefsButton, QuitButton};
use bevy::asset::AssetServer;
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::{
    default, ButtonBundle, Commands, Component, Entity, NodeBundle, Res, Text, TextAlignment,
    TextBundle, TextSection,
};

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: main_menu_style(),
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new("Memish", title_text_style(asset_server))],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
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
