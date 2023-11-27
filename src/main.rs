use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::window::{EnabledButtons, WindowResolution};
use memish::{AppState, MainMenuPlugin};

// TODO: make this a plugin
fn delayed_window(
    mut window: Query<&mut Window>,
    frames: Res<FrameCount>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if frames.0 == 3 {
        window.single_mut().visible = true;
        next_state.set(AppState::MainMenu);
    }
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn app_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Memish".to_string(),
            resizable: true,
            enabled_buttons: EnabledButtons {
                minimize: true,
                maximize: true,
                close: true,
            },
            visible: false,
            // position: WindowPosition::Centered(MonitorSelection::Current),
            resolution: WindowResolution::new(500.0, 250.0),
            ..default()
        }),
        ..default()
    }
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins.set(app_window_plugin()))
        .add_plugins(MainMenuPlugin)
        .add_systems(Startup, add_camera)
        .add_systems(Update, delayed_window.run_if(in_state(AppState::NoWindow)))
        .run()
}
