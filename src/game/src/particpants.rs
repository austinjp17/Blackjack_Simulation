
use std::sync::Arc;

use crate::betting_strategy::BettingFunc;
use crate::playing_strategy::{PlayerDecision, PlayingStrat, CountFunc};

use crate::{deck::{Hand, HandState}, GameState};

enum PlayerStrategy {
    DealerEmulation
}

// PLAYER
// #[derive(Clone, Debug)]
pub struct Player {
    pub hands: Vec<Hand>,
    pub playing_strat: Arc<PlayingStrat>,
    pub betting_strat: Arc<BettingFunc>,
    pub counting_strat: Arc<CountFunc>,
    pub consider_insurance: bool,
} 

impl Player {
    pub fn new(init_bet: u32, playing_strat: Arc<PlayingStrat>, 
        betting_strat: Arc<BettingFunc>, counting_strat: Arc<CountFunc>,
        consider_insurance: bool,
    ) -> Self { 
        Player { 
            hands: vec![Hand::new(init_bet)], 
            playing_strat,
            betting_strat,
            counting_strat,
            consider_insurance,
        } 
    }

    pub fn decide_bet(&self, game_state: GameState) -> u32 {
        (self.betting_strat)(game_state)
    }

    pub fn decide_play(&self, game_state: GameState) -> PlayerDecision {
        match &*self.playing_strat {
            PlayingStrat::Dealer(strat_fn) => strat_fn(game_state),
            PlayingStrat::Player(strat_fn) => strat_fn(game_state),
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
    pub strategy: Arc<PlayingStrat>
}

impl Dealer {
    pub fn new(cutoff: u8, strategy: PlayingStrat ) -> Self { 
        Dealer { hand: None, cutoff, strategy: Arc::new(strategy) }
    }

    // Dealer never doubles down, splits, or surrenders
    pub fn decide_play(&self, game_state: GameState) -> PlayerDecision {
        match &*self.strategy {
            PlayingStrat::Dealer(strat_fn) => strat_fn(game_state),
            PlayingStrat::Player(strat_fn) => strat_fn(game_state),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.hand.as_ref().expect("").state == HandState::Finished
    }

    pub fn set_cutoff(&mut self, new_cutoff: u8) { self.cutoff = new_cutoff }

}



// Player Strategies






