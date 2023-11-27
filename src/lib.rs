mod plugins;

use bevy::prelude::*;
pub use plugins::main_menu::MainMenuPlugin;
pub use plugins::startup::StartupPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
    // The start state before the window is displayed.
    #[default]
    NoWindow,

    // The main menu. This is displayed with the game starts after the window is displayed.
    MainMenu,
}
