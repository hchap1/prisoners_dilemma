mod game;
mod exchange;
mod agents;

use crate::agents::only_trust_once::OnlyTrustOnce;
use crate::agents::opposite_of_last::OppositeOfLast;
use crate::agents::Agent;

use crate::game::Game;
use crate::game::Settings;

fn main() {
    let only_trust_once = OnlyTrustOnce::spawn();
    let opposite_of_last = OppositeOfLast::spawn();

    let mut game = Game::new(Settings::default(), &only_trust_once, &opposite_of_last);
    let (a, b) = game.execute(100);
    println!("{a} | {b}");
}
