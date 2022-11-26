#![allow(dead_code)]
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub enum Action<'a> {
    Unknown,
    /// Provides a callback that takes in a set of cards (which will be random from the deck) and returns 2 cards back to the deck.
    Ambassador {
        select_role: fn([Class; 2]) -> [Class; 2],
    },
    Assinate {
        target_player: &'a Player,
    },
    Captain {
        target_player: &'a Player,
    },
    Coup {
        target_player: &'a Player,
    },
    Diplomat,
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
pub struct Deck {
    cards: Vec<Class>,
}

/// The Player class defines an individual player in the game.
/// 
/// Determines an action on its turn when notified by the engine.
/// Will have read-only access to public state of the game.
pub struct Player {
    cards: Vec<Class>,
    gold: i32,
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

impl Player {
    fn new() -> Self {
        Player {
            cards: Vec::new(),
            gold: 2,
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
    pub fn new(num_players: i32) -> Self {
        if num_players < 2 || num_players > 6 {
            panic!(
                "Invalid number of players specified. Value must be 2-6, got {}.",
                num_players
            );
        }

        let mut engine = Engine::default();
        engine.active_players = Vec::new();
        for _ in 0..num_players {
            engine.active_players.push(Player::new());
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_correctly() {
        let engine = Engine::new(5);
        assert_eq!(engine.active_players.len(), 5);
        assert_eq!(engine.eliminated_players.len(), 0);
        assert_eq!(engine.gold_pool, 30);
        assert_eq!(engine.turn_count, 0);
        assert_eq!(engine.card_pool.cards.len(), 5); // 15 cards total, minus 2 cards to each player
    }

    #[test]
    #[should_panic(expected = "Invalid number of players specified.")]
    fn panic_on_too_few_players() {
        Engine::new(1);
    }

    #[test]
    #[should_panic(expected = "Invalid number of players specified.")]
    fn panic_on_too_many_players() {
        Engine::new(7);
    }
}
