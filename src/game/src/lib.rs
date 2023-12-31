#![allow(dead_code)]
use std::sync::Arc;

use rand::Rng;

pub mod playing_strategy;
pub mod particpants;
pub mod deck;
pub mod run_hand;
pub mod actions;
pub mod run_many;
pub mod betting_strategy;

use crate::{
    particpants::{Player, Dealer},
    deck::{MultiDeck, Card, Hand, HandState},
    playing_strategy::{StrategyFunc, DealerUpcardStrength}
};

#[derive(Clone, Debug)]
pub struct EndState {
    pub hand_bet: u32,
    pub magnitude_bet_inc: u32,
    pub p_natural: bool,
    pub p_insurance: bool,
    pub p_doubled: bool,
    pub p_bust: bool,
    pub p_surrender_early: bool,
    pub p_surrender_late: bool,
    pub d_natural: bool,
    pub d_bust: bool,
}

impl Default for EndState {
    fn default() -> Self {
        EndState { 
            hand_bet: u32::MAX, magnitude_bet_inc: 0,
            p_natural: false, p_insurance: false, p_doubled: false, p_bust: false, p_surrender_early: false, p_surrender_late: false,
            d_natural: false, d_bust: false }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Winner {
    Player,
    Dealer,
    Tie,
    None
}

// #[derive(Clone, Debug)]
pub struct Game<R: Rng> {
    deck: MultiDeck,
    pub max_splits: u8,
    pub player: Player,
    pub dealer: Dealer,
    pub init_bet: u32,
    pub last_bet: u32,
    pub played_cards: Vec<Card>,
    pub last_winner: Winner,
    pub rng: R,
    pub echo: bool,

    // Surrender Rules
    pub allow_early_surrender: bool,
    pub allow_late_surrender: bool,

    // Counting
    pub running_count: i32,
    pub true_count: f64,
} 

impl <R: Rng + Clone> Game <R> {
    pub fn new (
        deck: MultiDeck,
        max_splits: u8, 
        init_bet: u32,
        dealer: Dealer,
        player: Player,
        rng: R,
        allow_early_surrender: bool,
        allow_late_surrender: bool,
        echo: bool,
        
        
    ) -> Self {
        Game { 
            deck: deck.clone(),
            max_splits, 
            init_bet,
            last_bet: 0,
            player,
            dealer,
            played_cards: vec![],
            last_winner: Winner::None,
            rng,
            echo,
            allow_early_surrender,
            allow_late_surrender,
            running_count: 0,
            true_count: 0.0,
            
        }
    }

    pub fn from_settings(value: Arc<GameSettings<R>>) -> Self {
        let player = Player {
            playing_strat: value.player_strat.clone(),
            betting_strat: value.betting_strat.clone(),
            counting_strat: value.counting_strat.clone(),
            insurance_strat: value.insurance_strat.clone(),
            hands: vec![],
        };
        let dealer = Dealer {
            strategy: value.dealer_strat.clone(),
            hand: None,
            cutoff: value.dealer_cutoff,
        };

        Game::new(value.deck.clone(), value.max_splits, value.init_bet, dealer, player, value.rng.clone(),value.allow_early_surrender, value.allow_late_surrender, value.echo)
    }
    
    // Assumes dealer has been dealth
    // Empty hand not handled
    pub fn get_dealer_upcard(&self) -> Option<Card> { 
        if let Some(hand) = self.dealer.hand.as_ref() {
            return hand.cards.first().copied();
        }
        None
    }

    pub fn get_dealer_upcard_str(&self) -> Option<DealerUpcardStrength> {
        if let Some(card) = self.get_dealer_upcard() {
            return Some(card.get_dealer_str());
        }
        None
        
        
    }

    pub fn get_state(&self, player_hand: Option<Hand>) -> GameState {
        GameState { 
            init_bet: self.init_bet,
            last_bet: self.last_bet,
            played_cards: self.played_cards.clone(),
            dealer_upcard: self.get_dealer_upcard(),
            dealer_upcard_str: self.get_dealer_upcard_str(),
            player_hand,
            dealer_hand: self.dealer.hand.clone(),
            dealer_cutoff: self.dealer.cutoff,
            contains_blank: self.deck.contains_blank,
            last_winner: self.last_winner.clone(),
            allow_early_surrender: self.allow_early_surrender,
            allow_late_surrender: self.allow_late_surrender, 
            running_count: self.running_count,
            true_count: self.true_count,
        }
    }

}


#[derive(Clone, Debug)]
pub struct GameState {
    init_bet: u32,
    last_bet: u32, // Martingale Strat
    played_cards: Vec<Card>, 
    dealer_upcard: Option<Card>,
    dealer_upcard_str: Option<DealerUpcardStrength>,
    player_hand: Option<Hand>,
    dealer_hand: Option<Hand>,
    dealer_cutoff: u8,
    contains_blank: bool,
    last_winner: Winner,
    // Surrenders Allowed
    allow_early_surrender: bool,
    allow_late_surrender: bool,
    

    // Card Counting
    running_count: i32,
    true_count: f64, // running_count.div(number of decks)

    
}

impl GameState {
    pub fn new(init_bet: u32, played_cards: Vec<Card>, dealer_upcard: Option<Card>, dealer_upcard_str: Option<DealerUpcardStrength>, 
        player_hand: Option<Hand>, dealer_hand: Option<Hand>, dealer_cutoff: u8, 
        contains_blank: bool, last_winner: Winner, running_count: i32, true_count: f64, allow_early_surrender:bool,
        allow_late_surrender: bool
    ) -> Self {
        GameState {
            init_bet,
            last_bet: 0,
            played_cards,
            dealer_upcard,
            dealer_upcard_str,
            player_hand,
            dealer_hand,
            dealer_cutoff,
            contains_blank,
            last_winner,
            allow_early_surrender,
            allow_late_surrender,
            running_count,
            true_count,

        }
    }
}

pub struct  GameSettings <R: Rng> {
    pub deck: MultiDeck,
    pub contains_blank: bool,
    pub max_splits: u8,
    pub init_bet: u32,
    pub dealer_cutoff: u8,
    pub dealer_strat: Arc<Box<dyn StrategyFunc>>,
    pub player_strat: Arc<Box<dyn StrategyFunc>>,
    pub betting_strat: Arc<Box<dyn StrategyFunc>>,
    pub counting_strat: Arc<Box<dyn StrategyFunc>>,
    pub insurance_strat: Arc<Box<dyn StrategyFunc>>,
    pub allow_early_surrender: bool,
    pub allow_late_surrender: bool,
    pub rng: R,
    pub echo: bool,
}


