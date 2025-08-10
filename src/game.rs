use crate::exchange::Exchange;
use crate::exchange::Action;
use crate::agents::Agent;

pub struct Settings {
    pub mutual_cooperation: usize,
    pub mutual_defection: usize,

    pub lose_standoff: usize,
    pub win_standoff: usize
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            mutual_cooperation: 3,
            mutual_defection: 1,
            lose_standoff: 0,
            win_standoff: 5
        }
    }
}

pub struct Game<'a, A: Agent, B: Agent> {
    a: &'a A,
    b: &'a B,
    a_so_far: Vec<Action>,
    b_so_far: Vec<Action>,
    settings: Settings
}

impl<'a, A: Agent, B: Agent> Game<'a, A, B> {

    pub fn new(settings: Settings, a: &'a A, b: &'a B) -> Self {
        Self { a, b, a_so_far: Vec::new(), b_so_far: Vec::new(), settings }
    }

    fn produce_records(&self) -> (Vec<Exchange>, Vec<Exchange>) {
        let mut for_a: Vec<Exchange> = Vec::new();
        let mut for_b: Vec<Exchange> = Vec::new();

        for idx in 0..self.a_so_far.len() {
            let (a_pov, b_pov) = Exchange::vice_versa(self.a_so_far[idx], self.b_so_far[idx]);
            for_a.push(a_pov);
            for_b.push(b_pov);
        }

        (for_a, for_b)
    }

    fn get_actions(&self) -> (Action, Action) {
        let (for_a, for_b) = self.produce_records();
        (self.a.choose(for_a), self.b.choose(for_b))
    }

    fn get_score(&self) -> (usize, usize) {
        let mut a_score: usize = 0;
        let mut b_score: usize = 0;

        for idx in 0..self.a_so_far.len() {
            let a_choice = self.a_so_far[idx];
            let b_choice = self.b_so_far[idx];

            a_score += a_choice.points(b_choice, &self.settings);
            b_score += b_choice.points(a_choice, &self.settings);
        }

        (a_score, b_score)
    }

    pub fn execute(&mut self, n: usize) -> (usize, usize) {
        for _ in 0..n {
            let (a_action, b_action) = self.get_actions();
            self.a_so_far.push(a_action);
            self.b_so_far.push(b_action);
        }

        self.get_score()
    }
}
