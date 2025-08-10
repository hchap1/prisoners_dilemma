mod game;
mod exchange;
mod agents;

use crate::agents::only_trust_once::OnlyTrustOnce;
use crate::agents::retaliate_once::RetaliateOnce;
use crate::agents::Agent;

use crate::game::Game;
use crate::game::Settings;

fn main() {
    let only_trust_once = OnlyTrustOnce::spawn();
    let retaliate_once = RetaliateOnce::spawn();

    let mut game = Game::new(Settings::default(), &only_trust_once, &retaliate_once);
    let (a, b) = game.execute(100);
    println!("{a} | {b}");
}
