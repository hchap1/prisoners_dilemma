use crate::exchange::Exchange;
use crate::exchange::Action;

pub trait Agent {
    fn choose(previous_moves: Vec<Exchange>) -> Action;
}
