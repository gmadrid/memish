use crate::plugins::prefs::layout::*;
use crate::plugins::prefs::*;
use crate::plugins::{
    HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR, SELECTED_BUTTON_COLOR,
    SELECTED_HOVERED_BUTTON_COLOR,
};
use crate::prefs::Prefs;

pub fn spawn_prefs_dialog(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    prefs: Res<Prefs>,
) {
    build_prefs_dialog(&mut commands, &asset_server, &prefs);
}

pub fn interact_with_basic_button(
    mut query: Query<(&Interaction, &mut CheckboxState, &PrefSetter), Changed<Interaction>>,
    mut evt_writer: EventWriter<PrefSetterEvent>,
) {
    for (interaction, mut button_state, setter) in query.iter_mut() {
        button_state.interaction = *interaction;
        if *interaction == Interaction::Pressed {
            evt_writer.send(PrefSetterEvent(setter.clone()))
        }
    }
}

pub fn read_pref_setter_events(mut events: EventReader<PrefSetterEvent>, mut prefs: ResMut<Prefs>) {
    for PrefSetterEvent(setter) in events.read() {
        match *setter {
            PrefSetter::Stack(val) => prefs.stack = val,
            PrefSetter::HalfStack => prefs.half_stack = !prefs.half_stack,
            PrefSetter::QuestionType(field) => match field {
                QuestionTypesField::CardToIndex => {
                    prefs.question_types.card_to_index = !prefs.question_types.card_to_index
                }
                QuestionTypesField::IndexToCard => {
                    prefs.question_types.index_to_card = !prefs.question_types.index_to_card
                }
                QuestionTypesField::NextCard => {
                    prefs.question_types.next_card = !prefs.question_types.next_card
                }
                QuestionTypesField::PreviousCard => {
                    prefs.question_types.previous_card = !prefs.question_types.previous_card
                }
            },
            PrefSetter::TimeLimit(val) => prefs.time_limit = val,
            PrefSetter::NumQuestions(val) => prefs.num_questions = val,
        }
    }
}

pub fn recompute_selected(
    mut old_prefs: Local<Option<Prefs>>,
    new_prefs: Res<Prefs>,
    mut query: Query<(&PrefSetter, &mut CheckboxState)>,
) {
    // TODO: add dirty bit to prefs so we can skip this absent a change.
    for (setter, mut checkbox_state) in query.iter_mut() {
        match setter {
            PrefSetter::Stack(setter_stack) => {
                if old_prefs
                    .as_ref()
                    .map(|op| op.stack != new_prefs.stack)
                    .unwrap_or(true)
                {
                    //if old_prefs.stack != new_prefs.stack {
                    checkbox_state.selected = *setter_stack == new_prefs.stack;
                }
            }
            PrefSetter::TimeLimit(setter_time_limit) => {
                if old_prefs
                    .as_ref()
                    .map(|op| op.time_limit != new_prefs.time_limit)
                    .unwrap_or(true)
                {
                    checkbox_state.selected = *setter_time_limit == new_prefs.time_limit;
                }
            }
            PrefSetter::NumQuestions(setter_num_questions) => {
                if old_prefs
                    .as_ref()
                    .map(|op| op.num_questions != new_prefs.num_questions)
                    .unwrap_or(true)
                {
                    checkbox_state.selected = *setter_num_questions == new_prefs.num_questions
                }
            }
            PrefSetter::HalfStack => {
                if old_prefs
                    .as_ref()
                    .map(|op| op.half_stack != new_prefs.half_stack)
                    .unwrap_or(true)
                {
                    checkbox_state.selected = new_prefs.half_stack;
                }
            }
            PrefSetter::QuestionType(setter_question_type) => {
                if old_prefs
                    .as_ref()
                    .map(|op| op.question_types == new_prefs.question_types)
                    .unwrap_or(true)
                {
                    match setter_question_type {
                        QuestionTypesField::CardToIndex => {
                            checkbox_state.selected = new_prefs.question_types.card_to_index
                        }
                        QuestionTypesField::IndexToCard => {
                            checkbox_state.selected = new_prefs.question_types.index_to_card
                        }
                        QuestionTypesField::NextCard => {
                            checkbox_state.selected = new_prefs.question_types.next_card
                        }
                        QuestionTypesField::PreviousCard => {
                            checkbox_state.selected = new_prefs.question_types.previous_card
                        }
                    }
                }
            }
        }
    }
    *old_prefs = Some(new_prefs.clone());
}

pub fn update_checkbox_display(
    mut query: Query<(&CheckboxState, &mut BackgroundColor), Changed<CheckboxState>>,
) {
    for (checkbox_state, mut background_color) in query.iter_mut() {
        let color = match checkbox_state.interaction {
            Interaction::Pressed => PRESSED_BUTTON_COLOR,
            Interaction::Hovered => {
                if checkbox_state.selected {
                    SELECTED_HOVERED_BUTTON_COLOR
                } else {
                    HOVERED_BUTTON_COLOR
                }
            }
            Interaction::None => {
                if checkbox_state.selected {
                    SELECTED_BUTTON_COLOR
                } else {
                    NORMAL_BUTTON_COLOR
                }
            }
        };
        *background_color = color.into();
    }
}
