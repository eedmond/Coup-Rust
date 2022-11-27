mod engine;
mod player_factory;

use crate::engine::Engine;

pub fn run_game(num_players: &Vec<String>) {
    let coup_game = Engine::new(player_factory::create_players(num_players));
    coup_game.print_deck();
    coup_game.print_current_actions();
}
