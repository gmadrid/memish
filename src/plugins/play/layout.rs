use crate::plugins::play::styles::*;
use crate::plugins::play::PlayScreen;
use crate::prefs::Prefs;
use bevy::prelude::*;

pub fn build_play_screen(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    prefs: &Res<Prefs>,
) {
    layout_play_container(commands, asset_server, prefs);
}

fn layout_play_container(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    prefs: &Res<Prefs>,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: play_screen_style(),
                background_color: Color::GRAY.into(),
                ..default()
            },
            PlayScreen,
        ))
        .with_children(|parent| {
            // layout_stack_checkboxes(parent, asset_server, prefs);
            // layout_half_stack_checkbox(parent, asset_server, prefs);
            // layout_question_type_selection(parent, asset_server, prefs);
            // layout_time_limit_checkboxes(parent, asset_server, prefs);
            // layout_max_questions_checkboxes(parent, asset_server, prefs);
        })
        .id()
}
