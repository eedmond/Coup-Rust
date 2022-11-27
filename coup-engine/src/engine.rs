#![allow(dead_code)]
use core::fmt::Debug;
use mset::MultiSet;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
pub enum Action<'a> {
    /// Provides a callback that takes in a set of cards (which will be random from the deck) and returns 2 cards back to the deck.
    Ambassador {
        select_role: fn([Option<Class>; 2]) -> [Option<Class>; 2],
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

#[derive(Debug, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum Class {
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
    role1: Option<Class>,
    role2: Option<Class>,
    gold: i32,
}

trait ActionImpl {
    fn handle_rebuttal(&self, rebuttal: &Action) {
        println!("This action cannot be rebutted!");
    }

    fn resolve_action(&mut self, engine: &mut Engine);
}

struct AmbassadorActionImpl<'a> {
    acting_player: &'a mut Player,
    select_role: fn([Option<Class>; 2]) -> [Option<Class>; 2],
}

impl<'a> ActionImpl for AmbassadorActionImpl<'a> {
    fn resolve_action(&mut self, engine: &mut Engine) {
        let card1 = engine.card_pool.draw_card();
        let card2 = engine.card_pool.draw_card();

        let returned_cards = (self.select_role)([card1.clone(), card2.clone()]);
        match self.get_selection(returned_cards, card1, card2) {
            Some(selection) => {
                self.return_card(selection.returned_cards[0].clone(), engine);
                self.return_card(selection.returned_cards[1].clone(), engine);
                self.acting_player.data.role1 = selection.chosen_cards[0].clone();
                self.acting_player.data.role2 = selection.chosen_cards[1].clone();
            }
            None => {
                println!(
                    "Player made invalid selection from ambassador. No roles will be changed."
                );
            }
        };
    }
}

struct AmbassadorCardSelection {
    chosen_cards: [Option<Class>; 2],
    returned_cards: [Option<Class>; 2],
}

impl<'a> AmbassadorActionImpl<'a> {
    fn new(
        acting_player: &'a mut Player,
        select_role: fn([Option<Class>; 2]) -> [Option<Class>; 2],
    ) -> Self {
        AmbassadorActionImpl {
            acting_player: acting_player,
            select_role: select_role,
        }
    }

    fn get_selection(
        &self,
        returned_cards: [Option<Class>; 2],
        drawn_card1: Option<Class>,
        drawn_card2: Option<Class>,
    ) -> Option<AmbassadorCardSelection> {
        let mut initial_roles_and_drawn_cards = MultiSet::new();
        initial_roles_and_drawn_cards.insert(self.acting_player.data.role1.clone());
        initial_roles_and_drawn_cards.insert(self.acting_player.data.role2.clone());
        initial_roles_and_drawn_cards.insert(drawn_card1);
        initial_roles_and_drawn_cards.insert(drawn_card2);

        let mut returned_cards_set = MultiSet::new();
        returned_cards_set.insert(returned_cards[0].clone());
        returned_cards_set.insert(returned_cards[1].clone());

        let new_roles = initial_roles_and_drawn_cards.difference(&returned_cards_set);

        Some(AmbassadorCardSelection {
            chosen_cards: [None, None],
            returned_cards: [None, None],
        })
    }

    fn return_card(&self, card: Option<Class>, engine: &mut Engine) {
        match card {
            Some(c) => engine.card_pool.return_card(c),
            None => {}
        };
    }
}

struct IncomeActionImpl<'a> {
    acting_player: &'a mut Player,
}

impl<'a> ActionImpl for IncomeActionImpl<'a> {
    fn resolve_action(&mut self, engine: &mut Engine) {
        let new_gold = cmp::min(2, engine.gold_pool);
        self.acting_player.data.gold += new_gold;
        engine.gold_pool -= new_gold;
    }
}

impl<'a> IncomeActionImpl<'a> {
    fn new(acting_player: &'a mut Player) -> Self {
        IncomeActionImpl {
            acting_player: acting_player,
        }
    }
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
    current_player_index: usize,
}

impl Default for PlayerData {
    fn default() -> Self {
        PlayerData {
            role1: None,
            role2: None,
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
            for _ in 0..3 {
                deck.cards.push(class.clone())
            }
        }

        deck.cards.shuffle(&mut thread_rng());
        deck
    }

    fn draw_card(&mut self) -> Option<Class> {
        self.cards.pop()
    }

    fn return_card(&mut self, card: Class) {
        self.cards.push(card);
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
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

    pub fn is_over(&self) -> bool {
        return self.active_players.len() > 1;
    }

    pub fn do_turn(&mut self) {
        let next_action = match self.active_players.get_mut(self.current_player_index) {
            Some(current_player) => match current_player.data.gold {
                10.. => Box::new(IncomeActionImpl::new(current_player)),
                _ => {
                    let next_action = current_player.controller.get_next_action();
                    Engine::get_action_impl(current_player, next_action)
                }
            },
            None => return,
        };
        // ask current player for action
        // ask other players for rebuttals
        //  if rebuttal: ask other players for
        // resolve action
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

    fn get_action_impl<'a>(
        acting_player: &'a mut Player,
        action: Action,
    ) -> Box<dyn ActionImpl + 'a> {
        return match action {
            Action::Ambassador { select_role } => {
                Box::new(AmbassadorActionImpl::new(acting_player, select_role))
            }
            _ => Box::new(IncomeActionImpl::new(acting_player)),
        };
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
            Action::Income
        }
    }
}
