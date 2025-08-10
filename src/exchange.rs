use crate::game::Settings;

#[derive(Clone, Copy, PartialEq)]
pub enum Action{
    Cooperate,
    Defect
}

impl Action {
    pub fn points(self, other: Self, settings: &Settings) -> usize {
        match self {
            Self::Cooperate => match other {
                Self::Cooperate => settings.mutual_cooperation,
                Self::Defect => settings.lose_standoff
            },
            Self::Defect => match other {
                Self::Cooperate => settings.win_standoff,
                Self::Defect => settings.mutual_defection
            }
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Self::Cooperate => Self::Defect,
            Self::Defect => Self::Cooperate
        }
    }
}

pub struct Exchange {
    pub this: Action,
    pub that: Action
}

impl Exchange {
    pub fn vice_versa(a_move: Action, b_move: Action) -> (Self, Self) {
        (Self { this: a_move, that: b_move }, Self { this: b_move, that: a_move })
    }
}
