use bevy::prelude::*;

pub fn prefs_dialog_style() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        flex_wrap: FlexWrap::Wrap,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::FlexStart,
        height: Val::Percent(100.0),
        width: Val::Percent(100.0),
        row_gap: Val::Px(16.0),
        column_gap: Val::Px(8.0),
        ..Style::DEFAULT
    }
}

pub fn prefs_subbox_style() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        height: Val::Auto, // Percent(100.0),
        width: Val::Auto,  // Percent(100.0),
        row_gap: Val::Px(8.0),
        column_gap: Val::Px(16.0),
        ..Style::DEFAULT
    }
}

pub fn subhead_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Ubuntu-Title.ttf"),
        font_size: 48.0,
        color: Color::YELLOW,
    }
}

pub fn checkbox_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/UbuntuCondensed-Regular.ttf"),
        font_size: 32.0,
        color: Color::ANTIQUE_WHITE,
    }
}
