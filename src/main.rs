use bevy::prelude::*;
use memish::{AppState, MainMenuPlugin, StartupPlugin};

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(StartupPlugin)
        .add_plugins(MainMenuPlugin)
        .run()
}
