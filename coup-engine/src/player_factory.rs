use crate::engine::Action;
use crate::engine::Engine;
use crate::engine::PlayerController;

pub fn create_players(player_strings: &Vec<String>) -> Vec<Box<dyn PlayerController>> {
    let mut players: Vec<Box<dyn PlayerController>> = Vec::new();
    for player_string in player_strings {
        match create_player(player_string) {
            Some(p) => players.push(p),
            None => println!("Unrecognized PlayerController requested: {}", player_string),
        }
    }

    players
}

// TODO: Figure out how to have external libraries add to this implementation without modifying this directly.
fn create_player(player_string: &String) -> Option<Box<dyn PlayerController>> {
    return match player_string.as_str() {
        "TempPlayer" => Some(Box::new(TempPlayerController::default())),
        _ => None,
    };
}

// Temporary code (until we have actual player controller implementations)
#[derive(Default, Clone)]
struct TempPlayerController;

impl PlayerController for TempPlayerController {
    fn init(&self, _engine: &Engine) {}

    fn get_next_action(&self) -> Action {
        Action::Duke
    }
}
// End temporary code
