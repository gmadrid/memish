use bevy::prelude::*;

mod layout;
mod plugin;
mod styles;
mod systems;

pub use plugin::PrefsPlugin;

#[derive(Component)]
struct PrefsDialog;

// The Checkbox value is `true` if the Checkbox is selected
#[derive(Component)]
struct Checkbox(bool);
