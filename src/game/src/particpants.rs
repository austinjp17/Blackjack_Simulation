
use std::sync::Arc;


use crate::playing_strategy::{StratReturn, PlayerDecision, CountFunc, StrategyFunc};

use crate::{deck::{Hand, HandState}, GameState};

enum PlayerStrategy {
    DealerEmulation
}

// PLAYER
// #[derive(Clone, Debug)]
pub struct Player {
    pub hands: Vec<Hand>,
    pub playing_strat: Arc<Box<dyn StrategyFunc>>,
    pub betting_strat: Arc<Box<dyn StrategyFunc>>,
    pub counting_strat: Arc<CountFunc>,
    pub insurance_strat: Arc<Box<dyn StrategyFunc>>,
} 

impl Player {
    pub fn new(init_bet: u32, playing_strat: Arc<Box<dyn StrategyFunc>>, 
        betting_strat: Arc<Box<dyn StrategyFunc>>, counting_strat: Arc<CountFunc>,
        insurance_strat: Arc<Box<dyn StrategyFunc>>,
    ) -> Self { 
        Player { 
            hands: vec![Hand::new(init_bet)], 
            playing_strat,
            betting_strat,
            counting_strat,
            insurance_strat,
        } 
    }

    pub fn decide_bet(&self, state: GameState) -> u32 { 
        match self.betting_strat.get_decision(state) {
            StratReturn::Bet(amt) => amt,
            _ => unreachable!("Always `bet` decision")
        } 
    }

    pub fn decide_insurance(&self, state: GameState) -> bool {
        match self.insurance_strat.get_decision(state) {
            StratReturn::Insurance(decision) => decision,
            _ => unreachable!("Always Insurnace variant")
        }
    }

    pub fn decide_play(&self, state: GameState) -> PlayerDecision { 
        match self.playing_strat.get_decision(state) {
            StratReturn::Play(decision) => decision,
            _ => unreachable!("Player decision always `Play` variant")
        }
    }

    pub fn is_finished(&self) -> bool { 
        self.hands.iter().all(|hand| hand.is_finished())
    }
    
}

// DEALER
// #[derive(Debug, Clone)]
pub struct Dealer {
    pub hand: Option<Hand>,
    pub cutoff: u8, //cutoff for dealer to stand
    pub strategy: Arc<Box<dyn StrategyFunc>>
}

impl Dealer {
    pub fn new(cutoff: u8, strategy: Arc<Box<dyn StrategyFunc>> ) -> Self { 
        Dealer { hand: None, cutoff, strategy }
    }

    // Dealer never doubles down, splits, or surrenders
    pub fn decide_play(&self, state: GameState) -> PlayerDecision {
        match self.strategy.get_decision(state) {
            StratReturn::Play(decision) => {decision},
            _ => {unreachable!("Dealer choice should always be `Play` variant")}
        }
    }

    pub fn is_finished(&self) -> bool {
        self.hand.as_ref().expect("").state == HandState::Finished
    }

    pub fn set_cutoff(&mut self, new_cutoff: u8) { self.cutoff = new_cutoff }

}



// Player Strategies






