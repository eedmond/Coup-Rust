mod engine;

use crate::engine::Action;
use crate::engine::Engine;
use crate::engine::PlayerController;

pub fn run_game(num_players: i32) {
    let coup_game = Engine::new(create_players(num_players));
    coup_game.print_deck();
    coup_game.print_current_actions();
}

// Temporary code (until we have actual player controller implementations)
fn create_players(num_players: i32) -> Vec<Box<dyn PlayerController>> {
    let test_player = TempPlayerController::default();

    let mut players: Vec<Box<dyn PlayerController>> = Vec::new();
    for _ in 0..num_players {
        players.push(Box::new(test_player.clone()));
    }

    players
}

#[derive(Default, Clone)]
struct TempPlayerController;

impl PlayerController for TempPlayerController {
    fn init(&self, _engine: &Engine) {

    }

    fn get_next_action(&self) -> Action {
        Action::Duke
    }
}
// End temporary code
