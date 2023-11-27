mod plugin;
mod systems;

use bevy::prelude::*;
pub use plugin::MainMenuPlugin;

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct PrefsButton;

#[derive(Component)]
struct QuitButton;
