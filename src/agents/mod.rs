use crate::exchange::Exchange;
use crate::exchange::Action;

pub trait Agent {
    fn spawn() -> Self where Self: Sized; 
    fn choose(&self, previous_moves: Vec<Exchange>) -> Action;
}

// Agents
pub mod opposite_of_last;
pub mod only_trust_once;
