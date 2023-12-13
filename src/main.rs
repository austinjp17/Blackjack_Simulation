#![allow(dead_code, unused_imports)]
use std::sync::Arc;




use game::run_many::GamePool;
use game::EndState;
use rand::prelude::*;
use rand::thread_rng;
use rand_chacha::ChaCha8Rng;
mod test;
use game::{
    deck::{Card, Hand, MultiDeck, Rank, Suit},
    playing_strategy::{StrategyFunc, 
        BasicStrategy, SplitOnly, DoubleOnly, CutoffOnly, // Basic Strat and it's main components
        DealerPlay, NaiveSoft, MimicDealer, 
        update_hi_lo, // Counting
        NoInsurance // Insurance
    },
    betting_strategy::{ConstantBet, Martingale},
    Game, GameSettings,
};


const MILLION: u64 = 1000000;

fn compare_strats(n: u64, strats: Vec<Box<dyn StrategyFunc>>) {

    for strat in strats {
        let rng = thread_rng();
        let deck = MultiDeck::new(6, true);
        let settings = Arc::new(GameSettings {
            deck,
            max_splits: 3,
            init_bet: 10,
            dealer_cutoff: 17,
            dealer_strat: Arc::new(Box::new(DealerPlay)),
            player_strat: Arc::new(strat),
            betting_strat: Arc::new(Box::new(ConstantBet)),
            counting_strat: Arc::new(Box::new(update_hi_lo)),
            insurance_strat: Arc::new(Box::new(NoInsurance)),
            allow_early_surrender: false,
            allow_late_surrender: false,
            contains_blank: true,
            rng,
            echo:false,
        });

        
                
        let mut pool = GamePool::new(settings.clone());
        pool.simulate(n, true);

        println!("\n--------------\n");
    }
}


fn main() {
    let n = MILLION;

    let strats:Vec<Box<dyn StrategyFunc>> = vec![
        Box::new(NaiveSoft), 
        Box::new(BasicStrategy)
        ]; 

    compare_strats(n*10, strats)

        
}
