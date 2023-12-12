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
    playing_strategy::{PlayingStrat, 
        basic_strategy, split_only, double_only, cutoff_only, // Basic Strat and it's main components
        dealer_hard_cutoff, naive_soft, mimic_dealer, 
        update_hi_lo, // Counting
        no_insurance // Insurance
    },
    betting_strategy::{constant_bet, martingale},
    Game, GameSettings,
};


const MILLION: u64 = 1000000;

fn compare_strats(n: u64) {
    let strats = [naive_soft]; 

    for strat in strats {
        let rng = thread_rng();
        let deck = MultiDeck::new(6, true);
        let settings = Arc::new(GameSettings {
            deck,
            max_splits: 3,
            init_bet: 10,
            dealer_cutoff: 17,
            dealer_strat: Arc::new(PlayingStrat::Dealer(Box::new(dealer_hard_cutoff))),
            player_strat: Arc::new(PlayingStrat::Player(Box::new(strat))),
            betting_strat: Arc::new(Box::new(constant_bet)),
            counting_strat: Arc::new(Box::new(update_hi_lo)),
            insurance_strat: Arc::new(Box::new(no_insurance)),
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
    // let rng = ChaCha8Rng::seed_from_u64(seed);

    let rng = thread_rng();
        let deck = MultiDeck::new(6, true);
        let settings = Arc::new(GameSettings {
            deck,
            max_splits: 3,
            init_bet: 10,
            dealer_cutoff: 17,
            dealer_strat: Arc::new(PlayingStrat::Dealer(Box::new(dealer_hard_cutoff))),
            player_strat: Arc::new(PlayingStrat::Player(Box::new(cutoff_only))),
            betting_strat: Arc::new(Box::new(constant_bet)),
            counting_strat: Arc::new(Box::new(update_hi_lo)),
            insurance_strat: Arc::new(Box::new(no_insurance)),
            allow_early_surrender: false,
            allow_late_surrender: false,
            contains_blank: true,
            rng,
            echo:false,
        });

        
                
        let mut pool = GamePool::new(settings.clone());
        pool.simulate(n, false);
}
