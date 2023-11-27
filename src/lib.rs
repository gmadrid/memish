mod main_menu;

use bevy::prelude::*;
pub use main_menu::MainMenuPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
    MainMenu,

    #[default]
    NoWindow,
}
