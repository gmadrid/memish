use bevy::prelude::*;

#[derive(Clone, Default, Resource, Debug)]
pub struct Prefs {
    pub stack: StackChoice,
    pub half_stack: bool,
    pub question_types: QuestionTypes,
    pub time_limit: TimeLimit,
    pub num_questions: NumQuestions,
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum StackChoice {
    #[default]
    Mnemonica,
    Memorandum,
    Aronson,
    Redford,
    Faro5,
}

// There are times in the UI where it's useful to pass a tag for a desired field
#[derive(Copy, Clone, Debug)]
pub enum QuestionTypesField {
    CardToIndex,
    IndexToCard,
    NextCard,
    PreviousCard,
}

#[derive(Clone, Debug)]
pub struct QuestionTypes {
    pub card_to_index: bool,
    pub index_to_card: bool,
    pub next_card: bool,
    pub previous_card: bool,
}

impl Default for QuestionTypes {
    fn default() -> Self {
        QuestionTypes {
            card_to_index: true,
            index_to_card: false,
            next_card: false,
            previous_card: false,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum TimeLimit {
    None,
    #[default]
    TenSeconds,
    FiveSeconds,
    ThreeSeconds,
    TwoSeconds,
    OneSecond,
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum NumQuestions {
    Forty,
    Twenty,
    #[default]
    Ten,
    Five,
}
