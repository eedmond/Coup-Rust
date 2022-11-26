mod engine;

use crate::engine::Engine;

pub fn run_game(num_players: i32) {
    let coup_game = Engine::new(num_players);
    coup_game.print_deck();
}