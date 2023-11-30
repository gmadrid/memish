use crate::prefs::Prefs;
use crate::AppState;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::window::{EnabledButtons, WindowResolution};

/// Setup
/// - the window
/// - delayed visibility to avoid a white flash
/// - the camera
/// - a Prefs resource
pub struct StartupPlugin;

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn delayed_window(
    mut window: Query<&mut Window>,
    frames: Res<FrameCount>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if frames.0 == 3 {
        window.single_mut().visible = true;
        //next_state.set(AppState::PrefsDialog);
        next_state.set(AppState::MainMenu);
    }
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
            resolution: WindowResolution::new(1000.0, 750.0),
            ..default()
        }),
        ..default()
    }
}

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(app_window_plugin()))
            .add_systems(Startup, add_camera)
            .add_systems(Update, delayed_window.run_if(in_state(AppState::NoWindow)))
            .init_resource::<Prefs>();
    }
}
