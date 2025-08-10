use crate::exchange::{Action, Exchange};

pub struct RetaliateOnce;

impl crate::agents::Agent for RetaliateOnce {
    fn spawn() -> Self { Self {} }
    fn choose(&self, previous_moves: Vec<Exchange>) -> Action {
        if let Some(previous_move) = previous_moves.last() {
            previous_move.that
        } else {
            Action::Cooperate
        }
    }
}
