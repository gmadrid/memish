use crate::plugins::prefs::styles::*;
use crate::plugins::prefs::*;
use crate::plugins::{button_style, NORMAL_BUTTON_COLOR};
use crate::prefs::{NumQuestions, Prefs, StackChoice, TimeLimit};

pub fn build_prefs_dialog(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    prefs: &Res<Prefs>,
) -> Entity {
    layout_container(commands, asset_server, prefs)
}

fn layout_container(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    prefs: &Res<Prefs>,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: prefs_dialog_style(),
                background_color: Color::GRAY.into(),
                ..default()
            },
            PrefsDialog,
        ))
        .with_children(|parent| {
            layout_stack_checkboxes(parent, asset_server, prefs);
            layout_half_stack_checkbox(parent, asset_server, prefs);
            layout_question_type_selection(parent, asset_server, prefs);
            layout_time_limit_checkboxes(parent, asset_server, prefs);
            layout_max_questions_checkboxes(parent, asset_server, prefs);
            layout_cancel_save_buttons(parent, asset_server, prefs);
        })
        .id()
}

fn layout_subbox(parent: &mut ChildBuilder, children: impl FnOnce(&mut ChildBuilder)) {
    layout_subbox_with_style(parent, prefs_subbox_style, children)
}

fn layout_subbox_with_style(
    parent: &mut ChildBuilder,
    style_fn: impl FnOnce() -> Style,
    children: impl FnOnce(&mut ChildBuilder),
) {
    parent
        .spawn(NodeBundle {
            style: style_fn(),
            ..default()
        })
        .with_children(children);
}

fn layout_cancel_save_buttons(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    prefs: &Res<Prefs>,
) {
    layout_subbox_with_style(
        parent,
        || Style {
            flex_direction: FlexDirection::Row,
            position_type: PositionType::Absolute,
            right: Val::Px(25.0),
            bottom: Val::Px(25.0),
            ..prefs_subbox_style()
        },
        |subbox| {
            layout_checkbox(
                subbox,
                "Cancel",
                asset_server,
                false,
                PrefSetter::CancelDialog,
            );
            layout_checkbox(subbox, "Save", asset_server, false, PrefSetter::SavePrefs);
        },
    );
}

fn layout_stack_checkboxes(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    prefs: &Res<Prefs>,
) {
    layout_subbox(parent, |subbox| {
        layout_subhead(subbox, "Stacks", asset_server);
        layout_checkbox(
            subbox,
            "Mnemonica",
            asset_server,
            prefs.stack == StackChoice::Mnemonica,
            PrefSetter::Stack(StackChoice::Mnemonica),
        );
        layout_checkbox(
            subbox,
            "Memorandum",
            asset_server,
            prefs.stack == StackChoice::Memorandum,
            PrefSetter::Stack(StackChoice::Memorandum),
        );
        layout_checkbox(
            subbox,
            "Aronson",
            asset_server,
            prefs.stack == StackChoice::Aronson,
            PrefSetter::Stack(StackChoice::Aronson),
        );
        layout_checkbox(
            subbox,
            "Redford",
            asset_server,
            prefs.stack == StackChoice::Redford,
            PrefSetter::Stack(StackChoice::Redford),
        );
        layout_checkbox(
            subbox,
            "Faro 5",
            asset_server,
            prefs.stack == StackChoice::Faro5,
            PrefSetter::Stack(StackChoice::Faro5),
        );
    });
}

fn layout_half_stack_checkbox(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    prefs: &Res<Prefs>,
) {
    layout_subbox(parent, |subbox| {
        layout_checkbox(
            subbox,
            "Half-stack",
            asset_server,
            prefs.half_stack,
            PrefSetter::HalfStack,
        );
    });
}

fn layout_question_type_selection(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    prefs: &Res<Prefs>,
) {
    layout_subbox(parent, |subbox| {
        layout_subhead(subbox, "Question\ntypes", asset_server);
        layout_checkbox(
            subbox,
            "Card to index",
            asset_server,
            prefs.question_types.card_to_index,
            PrefSetter::QuestionType(QuestionTypesField::CardToIndex),
        );
        layout_checkbox(
            subbox,
            "Index to card",
            asset_server,
            prefs.question_types.index_to_card,
            PrefSetter::QuestionType(QuestionTypesField::IndexToCard),
        );
        layout_checkbox(
            subbox,
            "Next card",
            asset_server,
            prefs.question_types.next_card,
            PrefSetter::QuestionType(QuestionTypesField::NextCard),
        );
        layout_checkbox(
            subbox,
            "Previous card",
            asset_server,
            prefs.question_types.previous_card,
            PrefSetter::QuestionType(QuestionTypesField::PreviousCard),
        );
    });
}

fn layout_time_limit_checkboxes(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    prefs: &Res<Prefs>,
) {
    layout_subbox(parent, |subbox| {
        layout_subhead(subbox, "Time limit", asset_server);
        layout_checkbox(
            subbox,
            "No limit",
            asset_server,
            prefs.time_limit == TimeLimit::None,
            PrefSetter::TimeLimit(TimeLimit::None),
        );
        layout_checkbox(
            subbox,
            "10 secs",
            asset_server,
            prefs.time_limit == TimeLimit::TenSeconds,
            PrefSetter::TimeLimit(TimeLimit::TenSeconds),
        );
        layout_checkbox(
            subbox,
            "5 secs",
            asset_server,
            prefs.time_limit == TimeLimit::FiveSeconds,
            PrefSetter::TimeLimit(TimeLimit::FiveSeconds),
        );
        layout_checkbox(
            subbox,
            "3 secs",
            asset_server,
            prefs.time_limit == TimeLimit::ThreeSeconds,
            PrefSetter::TimeLimit(TimeLimit::ThreeSeconds),
        );
        layout_checkbox(
            subbox,
            "2 secs",
            asset_server,
            prefs.time_limit == TimeLimit::TwoSeconds,
            PrefSetter::TimeLimit(TimeLimit::TwoSeconds),
        );
        layout_checkbox(
            subbox,
            "1 sec",
            asset_server,
            prefs.time_limit == TimeLimit::OneSecond,
            PrefSetter::TimeLimit(TimeLimit::OneSecond),
        );
    });
}

fn layout_max_questions_checkboxes(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    prefs: &Res<Prefs>,
) {
    layout_subbox(parent, |subbox| {
        layout_subhead(subbox, "Questions\nper game", asset_server);
        layout_checkbox(
            subbox,
            "40",
            asset_server,
            prefs.num_questions == NumQuestions::Forty,
            PrefSetter::NumQuestions(NumQuestions::Forty),
        );
        layout_checkbox(
            subbox,
            "20",
            asset_server,
            prefs.num_questions == NumQuestions::Twenty,
            PrefSetter::NumQuestions(NumQuestions::Twenty),
        );
        layout_checkbox(
            subbox,
            "10",
            asset_server,
            prefs.num_questions == NumQuestions::Ten,
            PrefSetter::NumQuestions(NumQuestions::Ten),
        );
        layout_checkbox(
            subbox,
            "5",
            asset_server,
            prefs.num_questions == NumQuestions::Five,
            PrefSetter::NumQuestions(NumQuestions::Five),
        );
    });
}

pub fn layout_subhead(parent: &mut ChildBuilder, label: &str, asset_server: &Res<AssetServer>) {
    parent.spawn(TextBundle {
        text: Text {
            sections: vec![TextSection::new(label, subhead_text_style(asset_server))],
            alignment: TextAlignment::Left,
            ..default()
        },
        ..default()
    });
}

pub fn layout_checkbox(
    parent: &mut ChildBuilder,
    label: &str,
    asset_server: &Res<AssetServer>,
    selected: bool,
    pref_setter: PrefSetter,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::FlexStart,
                    padding: UiRect::new(Val::Px(10.0), Val::DEFAULT, Val::DEFAULT, Val::DEFAULT),
                    ..button_style()
                },
                background_color: if selected {
                    Color::BLUE.into()
                } else {
                    NORMAL_BUTTON_COLOR.into()
                },
                ..default()
            },
            Checkbox,
            pref_setter,
            CheckboxState::default(),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(label, checkbox_text_style(asset_server))],
                    ..default()
                },
                ..default()
            });
        });
}
