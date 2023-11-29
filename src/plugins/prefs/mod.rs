mod layout;
mod plugin;
mod styles;
mod systems;

use crate::prefs::{NumQuestions, QuestionTypesField, StackChoice, TimeLimit};
use bevy::prelude::*;

pub use plugin::PrefsPlugin;

#[derive(Component)]
struct PrefsDialog;

// The Checkbox value is `true` if the Checkbox is selected
#[derive(Component)]
struct Checkbox(bool);

#[derive(Component)]
struct ButtonState {
    pub selected: bool,
    pub hovered: bool,
    pub ghosted: bool,
}

#[derive(Component, Debug, Clone)]
enum PrefSetter {
    Stack(StackChoice),
    HalfStack(bool),
    QuestionType(QuestionTypesField, bool),
    TimeLimit(TimeLimit),
    NumQuestions(NumQuestions),
}

#[derive(Event)]
struct PrefSetterEvent(PrefSetter);
