use crate::exchange::Exchange;
use crate::exchange::Action;

pub trait Agent {
    fn choose(previous_moves: Vec<Exchange>) -> Action;
}

// Agents
pub mod opposite_of_last;
