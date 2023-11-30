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
struct Checkbox;

#[derive(Component, Default, Eq, PartialEq, Clone, Copy)]
struct CheckboxState {
    pub interaction: Interaction,
    pub selected: bool,
}

// TODO: rename or restructure this because it's now used for more than just pref-setting.
#[derive(Component, Debug, Clone)]
enum PrefSetter {
    // Selections
    Stack(StackChoice),
    TimeLimit(TimeLimit),
    NumQuestions(NumQuestions),

    // Toggles
    HalfStack,
    QuestionType(QuestionTypesField),

    // Actions
    CancelDialog,
    SavePrefs,
}

#[derive(Event)]
struct PrefSetterEvent(PrefSetter);
