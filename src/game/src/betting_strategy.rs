use crate::{GameState, Winner};

pub type BettingFunc = Box<dyn Fn(GameState) -> u32>;

pub fn constant_bet(state: GameState) -> u32 { state.init_bet }

pub fn martingale(state: GameState) -> u32 {
    // Goal: Win payout should cover all previous losses 
    // If lost last hand, expontially increase bet
    // If Win: Reset bet to init
    // If Tied: Bet same amount
    
    match state.last_winner {
        Winner::Player => {state.init_bet }, // Reset bet
        Winner::Dealer => { state.last_bet * 2 }, // Increase bet
        Winner::Tie => {state.last_bet},
        Winner::None => {state.init_bet}
    }

}



