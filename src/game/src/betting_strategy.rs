use crate::{GameState, Winner, StrategyFunc, playing_strategy::StratReturn};

pub struct ConstantBet;
impl StrategyFunc for ConstantBet {
    fn get_decision(&self, state: GameState) -> StratReturn {
        StratReturn::Bet(state.init_bet)
    }

    fn to_string(&self) -> String {
        "Constant Bet".to_string()
    }
}

pub struct Martingale;
impl StrategyFunc for Martingale {
    fn get_decision(&self, state: GameState) -> StratReturn {
        // Goal: Win payout should cover all previous losses 
        // If lost last hand, expontially increase bet
        // If Win: Reset bet to init
        // If Tied: Bet same amount
        
        let next_bet = match state.last_winner {
            Winner::Dealer => { state.last_bet * 2 }, // Increase bet
            Winner::Player => {state.init_bet }, // Reset bet
            Winner::Tie => { state.last_bet}, // Rebet same amount
            Winner::None => { state.init_bet}
        };

        StratReturn::Bet(next_bet)
    }

    fn to_string(&self) -> String {
        todo!()
    }
}


// percent
pub fn kelly_criterion(state: GameState) -> u32 { state.init_bet }

