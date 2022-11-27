mod engine;
mod player_factory;

use crate::engine::Engine;

pub fn run_game(num_players: &Vec<String>) {
    let mut coup_game = Engine::new(player_factory::create_players(num_players));
    while !coup_game.is_over() {
        coup_game.do_turn();
    }

    coup_game.print_deck();
    coup_game.print_current_actions();
}
