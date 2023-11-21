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
    playing_strategy::{basic_strategy, dealer_strat, naive_hard, naive_soft, PlayingStrat, update_hi_lo},
    betting_strategy::{constant_bet, martingale},
    Game, GameSettings,
};


fn main() {
    let rng = thread_rng();
    let deck = MultiDeck::new(6, true);
    let settings = GameSettings {
        deck,
        max_splits: 3,
        init_bet: 10,
        dealer_cutoff: 17,
        dealer_strat: Arc::new(PlayingStrat::Dealer(Box::new(dealer_strat))),
        player_strat: Arc::new(PlayingStrat::Player(Box::new(naive_soft))),
        player_betting_strat: Arc::new(Box::new(constant_bet)),
        counting_strat: Arc::new(Box::new(update_hi_lo)),
        consider_insurance: false,
        contains_blank: true,
        rng,
        echo:false,
    };

    let mut pool = GamePool::new(Arc::new(settings));
    pool.simulate(1000000);

    // let rng = ChaCha8Rng::seed_from_u64(seed);
}
