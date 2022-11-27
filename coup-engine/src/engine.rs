#![allow(dead_code)]
use core::fmt::Debug;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
pub enum Action<'a> {
    Unknown,
    /// Provides a callback that takes in a set of cards (which will be random from the deck) and returns 2 cards back to the deck.
    Ambassador {
        select_role: fn([Class; 2]) -> [Class; 2],
    },
    Assassinate {
        target_player: &'a Player,
    },
    Captain {
        target_player: &'a Player,
    },
    Coup {
        target_player: &'a Player,
    },
    Duke,
    ForeignAid,
    Income,
}

#[derive(Debug, Clone, EnumIter, PartialEq)]
pub enum Class {
    Unknown,
    Ambassador,
    Assassin,
    Captain,
    Contessa,
    Duke,
}

#[derive(Default)]
struct Deck {
    cards: Vec<Class>,
}

#[derive(Debug)]
struct PlayerData {
    cards: Vec<Class>,
    gold: i32,
}

pub trait PlayerController {
    fn init(&self, engine: &Engine);
    fn get_next_action(&self) -> Action;
}

impl Debug for dyn PlayerController {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PlayerController")
    }
}
/// The Player class defines an individual player in the game.
///
/// Determines an action on its turn when notified by the engine.
/// Will have read-only access to public state of the game.
#[derive(Debug)]
pub struct Player {
    data: PlayerData,
    controller: Box<dyn PlayerController>,
}

/// The Engine class runs the Coup game.
///
/// It is responsible for coordinating each turn of the game. It will ask players for their action, ensure the action is valid, update state accordingly, and determine when the game is over.
#[derive(Default)]
pub struct Engine {
    active_players: Vec<Player>,
    turn_count: i32,
    card_pool: Deck,
    gold_pool: i32,
    eliminated_players: Vec<Player>,
}

impl Default for PlayerData {
    fn default() -> Self {
        PlayerData {
            cards: Vec::new(),
            gold: 2,
        }
    }
}

impl Player {
    fn new(controller: Box<dyn PlayerController>) -> Self {
        Player {
            data: PlayerData::default(),
            controller: controller,
        }
    }
}

impl Deck {
    fn new() -> Self {
        let mut deck = Deck::default();
        deck.cards = Vec::new();
        for class in Class::iter() {
            if class == Class::Unknown {
                continue;
            }

            for _ in 0..3 {
                deck.cards.push(class.clone())
            }
        }

        deck.cards.shuffle(&mut thread_rng());
        deck
    }
}

impl Engine {
    pub fn new(players: Vec<Box<dyn PlayerController>>) -> Self {
        if players.len() < 2 || players.len() > 6 {
            panic!(
                "Invalid number of players specified. Value must be 2-6, got {}.",
                players.len()
            );
        }

        let mut engine = Engine::default();

        engine.active_players = Vec::new();
        for controller in players {
            engine.active_players.push(Player::new(controller));
        }

        engine.turn_count = 0;
        engine.card_pool = Deck::new();
        engine.gold_pool = 30;

        engine
    }

    pub fn print_deck(&self) {
        for card in &self.card_pool.cards {
            println!("Card: {:?}.", card);
        }
    }

    pub fn print_current_actions(&self) {
        for player in &self.active_players {
            println!("First action: {:?}", player.controller.get_next_action());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_correctly() {
        let engine = Engine::new(create_players(5));
        assert_eq!(engine.active_players.len(), 5);
        assert_eq!(engine.eliminated_players.len(), 0);
        assert_eq!(engine.gold_pool, 30);
        assert_eq!(engine.turn_count, 0);
        assert_eq!(engine.card_pool.cards.len(), 5); // 15 cards total, minus 2 cards to each player
    }

    #[test]
    #[should_panic(expected = "Invalid number of players specified.")]
    fn panic_on_too_few_players() {
        Engine::new(create_players(1));
    }

    #[test]
    #[should_panic(expected = "Invalid number of players specified.")]
    fn panic_on_too_many_players() {
        Engine::new(create_players(7));
    }

    fn create_players(num_players: i32) -> Vec<Box<dyn PlayerController>> {
        let test_player = TestPlayerController::default();

        let mut players: Vec<Box<dyn PlayerController>> = Vec::new();
        for _ in 0..num_players {
            players.push(Box::new(test_player.clone()));
        }

        players
    }

    #[derive(Default, Clone)]
    struct TestPlayerController;

    impl PlayerController for TestPlayerController {
        fn init(&self, _engine: &Engine) {}

        fn get_next_action(&self) -> Action {
            Action::Unknown
        }
    }
}
