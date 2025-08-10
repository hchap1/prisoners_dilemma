use crate::exchange::{Action, Exchange};

pub struct OnlyTrustOnce;

/*
Always cooperate, but as soon as the enemy defects, always defect.
*/

impl crate::agents::Agent for OnlyTrustOnce {
    fn spawn() -> Self { Self {} }
    fn choose(&self, previous_moves: Vec<Exchange>) -> Action {
        if previous_moves.iter().any(|x| x.that == Action::Defect) {
            Action::Defect
        } else {
            Action::Cooperate
        }
    }
}
