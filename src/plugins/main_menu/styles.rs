use bevy::prelude::*;

pub fn main_menu_style() -> Style {
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

pub fn title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Ubuntu-Title.ttf"),
        font_size: 64.0,
        color: Color::YELLOW,
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/UbuntuCondensed-Regular.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}
