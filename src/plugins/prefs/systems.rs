use crate::plugins::prefs::layout::*;
use crate::plugins::prefs::*;
use crate::plugins::{interact_with_button, NORMAL_BUTTON_COLOR, SELECTED_BUTTON_COLOR};
use crate::prefs::Prefs;

pub fn spawn_prefs_dialog(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    prefs: Res<Prefs>,
) {
    build_prefs_dialog(&mut commands, &asset_server, &prefs);
}

pub fn interact_with_checkbox(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &Checkbox, &PrefSetter),
        Changed<Interaction>,
    >,
    mut evt_writer: EventWriter<PrefSetterEvent>,
) {
    for (interaction, mut background_color, Checkbox(selected), setter) in query.iter_mut() {
        interact_with_button(interaction, &mut background_color, *selected);
        if *interaction == Interaction::Pressed {
            evt_writer.send(PrefSetterEvent(setter.clone()))
        }
    }
}

pub fn update_selected(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Checkbox, &PrefSetter)>,
    prefs: Res<Prefs>,
) {
    // TODO: can we put in a check for changes to the Prefs?
    for (entity, mut checkbox, setter) in query.iter_mut() {
        match *setter {
            PrefSetter::Stack(stack) => {
                let selected = stack == prefs.stack;
                if checkbox.0 != selected {
                    checkbox.0 = selected;
                    let color = if selected {
                        SELECTED_BUTTON_COLOR
                    } else {
                        NORMAL_BUTTON_COLOR
                    }
                    .into();
                    commands.entity(entity).insert(BackgroundColor(color));
                }
            }
            PrefSetter::HalfStack(_) => {}
            PrefSetter::QuestionType(_, _) => {}
            PrefSetter::TimeLimit(_) => {}
            PrefSetter::NumQuestions(_) => {}
        }
    }
}

pub fn read_pref_setter_events(mut events: EventReader<PrefSetterEvent>, mut prefs: ResMut<Prefs>) {
    for PrefSetterEvent(setter) in events.read() {
        match *setter {
            PrefSetter::Stack(val) => prefs.stack = val,
            PrefSetter::HalfStack(val) => prefs.half_stack = val,
            PrefSetter::QuestionType(field, val) => match field {
                QuestionTypesField::CardToIndex => prefs.question_types.card_to_index = val,
                QuestionTypesField::IndexToCard => prefs.question_types.index_to_card = val,
                QuestionTypesField::NextCard => prefs.question_types.next_card = val,
                QuestionTypesField::PreviousCard => prefs.question_types.previous_card = val,
            },
            PrefSetter::TimeLimit(val) => prefs.time_limit = val,
            PrefSetter::NumQuestions(val) => prefs.num_questions = val,
        }
    }
}
