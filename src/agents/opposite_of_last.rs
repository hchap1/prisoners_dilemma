use crate::exchange::{Action, Exchange};

pub struct OppositeOfLast;

impl crate::agents::Agent for OppositeOfLast {
    fn choose(previous_moves: Vec<Exchange>) -> Action {
        if let Some(previous_move) = previous_moves.last() {
            previous_move.that.opposite()
        } else {
            Action::Cooperate
        }
    }
}
