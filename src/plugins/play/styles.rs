use bevy::prelude::*;

pub fn play_screen_style() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        flex_wrap: FlexWrap::NoWrap,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::FlexStart,
        height: Val::Percent(100.0),
        width: Val::Percent(100.0),
        row_gap: Val::ZERO,
        column_gap: Val::ZERO,
        ..Style::DEFAULT
    }
}
