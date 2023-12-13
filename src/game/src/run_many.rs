use std::ops::Div;
use std::sync::Arc;
use rand::Rng;
use indicatif::ProgressBar;
use crate::deck::Hand;
use crate::{Game, GameSettings, Winner, EndState, playing_strategy};
use std::time::Instant;
use std::sync::atomic::{AtomicBool, Ordering};
use ctrlc;
use num_format::{Locale, ToFormattedString};

macro_rules! log_fn {
    ($func:ident) => {
        { stringify!($func) }
    };
}

pub struct GamePool <R: Rng> {
    pub settings: Arc<GameSettings<R>>,
    pub results: Vec<(Winner, EndState)>,
    pub simulated_games: u64,
}

impl <R:Rng + Clone> GamePool <R> {
    pub fn new(settings: Arc<GameSettings<R>>) -> Self {
        Self { settings, results: vec![], simulated_games: 0 }
    }

    pub fn simulate(&mut self, n: u64, progress_bar: bool) {
        let mut bj = Game::from_settings(self.settings.clone());
        
        let bar = ProgressBar::new(n);
        
        for _ in 0..n {
            self.simulated_games += 1;
            // Player decides init bet
            // Filler hand passed
            let hand_bet = bj.player.decide_bet(bj.get_state(Some(Hand::new(bj.init_bet))));
            

            // Deal cards after bet decided
            bj.deal(hand_bet);

            // Append Hand Results & incriment run count
            self.results.append(&mut bj.play_hand());
            
            // Empty Hands
            bj.reset_hands();
            
            if progress_bar {    
                bar.inc(1); // Progress Bar
            }
            
        }
        if progress_bar {
            bar.finish();
        }
        

        self.sum_results();
        
    }

    pub fn debug_simulate(&mut self, n: u64) {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");

    
        let start = Instant::now();
        let mut bj = Game::from_settings(self.settings.clone());
        let settings_time = start.elapsed();
    
        let bar = ProgressBar::new(n);
        let mut deal_time_total = std::time::Duration::new(0, 0);
        let mut play_hand_time_total = std::time::Duration::new(0, 0);
        let mut reset_time_total = std::time::Duration::new(0, 0);
    
        for i in 0..n {
            // Cntl-C Break
            if !running.load(Ordering::SeqCst) {
                println!("Simulation interrupted at game {}", i);
                break;
            }
            // Timer Start
            let deal_start = Instant::now();
            bj.deal(bj.init_bet);
            deal_time_total += deal_start.elapsed();
    
            let play_hand_start = Instant::now();
            let mut res = bj.play_hand();
            play_hand_time_total += play_hand_start.elapsed();
    
            self.results.append(&mut res);
    
            let reset_start = Instant::now();
            bj.reset_hands();
            reset_time_total += reset_start.elapsed();
    
            bar.inc(1); // Progress Bar
        }
    
        bar.finish();
    
        let sum_results_start = Instant::now();
        self.sum_results();
        let sum_results_time = sum_results_start.elapsed();
    
        let total_time = start.elapsed();
    
        println!("Settings clone time: {:?}", settings_time);
        println!("Average deal time: {:?}", deal_time_total / (n as u32));
        println!("Average play hand time: {:?}", play_hand_time_total / (n as u32));
        println!("Average reset time: {:?}", reset_time_total / (n as u32));
        println!("Sum results time: {:?}", sum_results_time);
        println!("Total simulation time: {:?}", total_time);
    } 

    // ---RESULT FUNCTIONS---

    pub fn get_player_wins(&self) -> Vec<(Winner, EndState)> {
        self.results.iter().filter(|(winner, _)| winner == &Winner::Player)
        .map(|(winner, state)| (winner.clone(), state.clone()) )
        .collect::<Vec<(Winner, EndState)>>()
        
    }

    pub fn get_dealer_wins(&self) -> Vec<(Winner, EndState)> {
        self.results.iter().filter(|(winner, _)| winner == &Winner::Dealer)
        .map(|(winner, state)| (winner.clone(), state.clone()) )
        .collect::<Vec<(Winner, EndState)>>()
    }

    pub fn get_ties(&self) -> Vec<(Winner, EndState)> {
        self.results.iter().filter(|(winner, _)| winner == &Winner::Tie)
        .map(|(winner, state)| (winner.clone(), state.clone()) )
        .collect::<Vec<(Winner, EndState)>>()
    }

    pub fn get_player_payoff(&self) -> i64 {
        let mut total_payoff: i64 = 0;
        self.results.iter().for_each(|(winner, state)| {
            let mut hand_payoff: i32 = 0;
            match winner {
                Winner::Player => {
                    // Player Natural && Not Dealer Natural
                    if state.p_natural && !state.d_natural {
                        hand_payoff += (state.hand_bet * 3).div(2) as i32 // Natural BJ pays out 3/2 of bet
                    }

                    // If player doubled
                    else if state.p_doubled {
                        hand_payoff += state.hand_bet as i32 * 2
                    }

                    // Insurance
                    else if state.p_insurance {}

                }
                Winner::Dealer => {hand_payoff -= state.hand_bet as i32}
                Winner::Tie => {}
                Winner::None => {}
            }
            total_payoff += hand_payoff as i64;
        });

        total_payoff
    }

    pub fn sum_player_stats(&self) {
        let num_loses = self.results.iter().filter(|(winner, _)| { winner == &Winner::Dealer }).collect::<Vec<&(Winner, EndState)>>().len() as f64;
        let num_bust = self.results.iter().filter(|(_, state)| {state.p_bust}).collect::<Vec<&(Winner, EndState)>>().len() as f64;
        let perc_bust = 100_f64*num_bust.div(self.simulated_games as f64);
        let percent_bust_given_loss = 100_f64*num_bust.div(num_loses);
        println!("Player Stats");
        println!(" - Percent Bust: {}%", perc_bust);
        println!(" - Percent Bust|Loss: {}%", percent_bust_given_loss);
    }

    pub fn sum_results(&self) {
        #[allow(unused_variables)] // Used for fn name
        let player_strat = self.settings.player_strat.as_ref().as_ref();

        let player_wins: u64 = self.results.iter()
        .map(|(winner, _)| match winner { Winner::Player => {1} _ => {0} })
        .collect::<Vec<u64>>().iter().sum();

        let num_ties: u64 = self.results.iter()
        .map(|(winner, _)| match winner { Winner::Tie => {1} _ => {0} })
        .collect::<Vec<u64>>().iter().sum();

        let dealer_wins = (self.results.len() as u64) - (player_wins + num_ties);
        

        // Format Large Numbers
        let game_count_str = self.results.len().to_formatted_string(&Locale::en);
        let dealer_wins_str = dealer_wins.to_formatted_string(&Locale::en);
        let player_wins_str = player_wins.to_formatted_string(&Locale::en);
        let num_ties_str = num_ties.to_formatted_string(&Locale::en);
        let payoff_str = self.get_player_payoff().to_formatted_string(&Locale::en);
        let player_strat_str = player_strat.to_string();

        // |-------------------------|
        // |      Results Output     |
        // |-------------------------|
        println!("\n -- Simulation Results --\n");
        // Settings
        println!("Player Strat: {}", player_strat_str);
        println!("n = {}", game_count_str);

        // Results
        println!("Player Wins: {} | {}%", player_wins_str, (100_f64*(player_wins as f64).div(self.results.len() as f64)));
        println!("Dealer Wins: {} | {}%", dealer_wins_str, (100_f64*(dealer_wins as f64).div(self.results.len() as f64)));
        println!("Ties: {} | {}%", num_ties_str, (100_f64*(num_ties as f64).div(self.results.len() as f64)));
        println!("Player Payoff: ${}", payoff_str);
        println!("Player Payoff/Game: ${}", (self.settings.init_bet as f64 * self.get_player_payoff() as f64).div(self.results.len() as f64));
        self.sum_player_stats();
    }
}



