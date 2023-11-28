use bevy::prelude::*;

pub mod main_menu;
pub mod prefs;
pub mod startup;

const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);
const SELECTED_BUTTON_COLOR: Color = Color::MIDNIGHT_BLUE;

fn despawn_entity<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

fn interact_with_button(
    interaction: &Interaction,
    background_color: &mut BackgroundColor,
    selected: bool,
) {
    match *interaction {
        Interaction::Pressed => {
            *background_color = PRESSED_BUTTON_COLOR.into();
        }
        Interaction::Hovered => {
            *background_color = HOVERED_BUTTON_COLOR.into();
        }
        Interaction::None => {
            *background_color = if selected {
                SELECTED_BUTTON_COLOR.into()
            } else {
                NORMAL_BUTTON_COLOR.into()
            };
        }
    };
}

pub fn button_style() -> Style {
    Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        height: Val::Px(80.0),
        width: Val::Px(200.0),
        ..Style::DEFAULT
    }
}
