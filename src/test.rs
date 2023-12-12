use rand::Rng;
use std::sync::Arc;

use crate::*;

#[cfg(test)]
mod tests {
    use game::{playing_strategy::{PlayerDecision, update_knock_out, omega_2, StrategyFunc, CountFunc, InsuranceFunc, card_counter_insurance}, Winner, deck::HandState, betting_strategy::{BettingFunc, self}};

    use super::*;
    

    // Test Helpers
    fn standard_game (
        player_strat: Option<Arc<PlayingStrat>>, 
        dealer_strategy: Option<Arc<PlayingStrat>>, 
        betting_strat: Option<Arc<BettingFunc>>, 
        counting_strat: Option<Arc<CountFunc>>,
        insurance_strat: Option<Arc<InsuranceFunc>>,
    ) -> Game<ChaCha8Rng> {

        let player_strat = match player_strat {
            Some(strat) => strat,
            None => Arc::new(PlayingStrat::Player(Box::new(basic_strategy)))
        };

        let dealer_strat: Arc<PlayingStrat> = match dealer_strategy {
            Some(strat) => strat,
            None => Arc::new(PlayingStrat::Dealer(Box::new(dealer_hard_17)))
        };

        let betting_strat: Arc<BettingFunc> = match betting_strat {
            Some(strat) => strat,
            None => Arc::new(Box::new(constant_bet))
        };

        let counting_strat: Arc<CountFunc> = match counting_strat {
            Some(strat) => strat,
            None => Arc::new(Box::new(update_hi_lo)),
        };

        let insurance_strat: Arc<InsuranceFunc> = match insurance_strat {
            Some(strat) => strat,
            None => Arc::new(Box::new(no_insurance)),
        };
        
        let mut rng = ChaCha8Rng::seed_from_u64(2);
        let mut deck = MultiDeck::new(6, false);
        deck.shuffle(&mut rng);

        // Set Game Settings
        let settings = GameSettings {
            deck,
            max_splits: 3,
            contains_blank: true,
            init_bet: 10,
            dealer_cutoff: 17,
            dealer_strat,
            player_strat,
            betting_strat,
            counting_strat,
            insurance_strat,
            allow_early_surrender: false,
            allow_late_surrender: false,    
            echo: false,
            rng,
        };

        Game::from_settings(Arc::new(settings))
    }
    
    fn set_hands <R: Rng + Clone>(
        game: &mut Game<R>,
        dealer_hand: Hand,
        player_hands: Vec<Hand>,
    ) { 
        // Set hands & Pass back
        game.set_player_hands(player_hands);
        game.set_dealer_hand(dealer_hand);
        
    }

    fn test_decision(dealer_hand: Hand, player_hands: Vec<Hand>, expected_decision: PlayerDecision) -> Arc<Game<ChaCha8Rng>> {
        let mut rng = ChaCha8Rng::seed_from_u64(2);
        let mut deck = MultiDeck::new(6, false);
        deck.shuffle(&mut rng);

        // Set Game Settings
        let settings = GameSettings {
            deck,
            max_splits: 3,
            contains_blank: true,
            init_bet: 10,
            dealer_cutoff: 17,
            dealer_strat: Arc::new(PlayingStrat::Dealer(Box::new(dealer_hard_17))),
            player_strat: Arc::new(PlayingStrat::Player(Box::new(basic_strategy))),
            betting_strat: Arc::new(Box::new(constant_bet)),
            counting_strat: Arc::new(Box::new(update_hi_lo)),
            insurance_strat: Arc::new(Box::new(no_insurance)),
            allow_early_surrender: false,
            allow_late_surrender: false,
            echo: false,
            rng,
        };

        let mut test_game = Game::from_settings(Arc::new(settings));

        set_hands(&mut test_game, dealer_hand, player_hands.clone());

        let state = test_game.get_state(&player_hands[0]);

        let decision = test_game.player.decide_play(state);

        assert_eq!(decision, expected_decision);

        test_game.play_hand();

        Arc::new(test_game).clone()
    }



// |-------------------------|
// |     Game Setup Tests    |
// |-------------------------|

    #[test]
    fn game_creation() {
        let mut rng = ChaCha8Rng::seed_from_u64(2);
        let mut deck = MultiDeck::new(6, false);
        deck.shuffle(&mut rng);

        // Set Game Settings
        let settings = GameSettings {
            deck,
            max_splits: 3,
            contains_blank: false,
            init_bet: 10,
            dealer_cutoff: 17,
            dealer_strat: Arc::new(PlayingStrat::Dealer(Box::new(dealer_hard_17))),
            player_strat: Arc::new(PlayingStrat::Player(Box::new(basic_strategy))),
            betting_strat: Arc::new(Box::new(constant_bet)),
            counting_strat: Arc::new(Box::new(update_hi_lo)),
            insurance_strat: Arc::new(Box::new(no_insurance)),
            allow_early_surrender: false,
            allow_late_surrender: false,
            echo: false,
            rng,
        };

        let bj = Game::from_settings(Arc::new(settings));

        assert!(bj.played_cards.is_empty());
        assert!(bj.dealer.hand.is_none());
        assert!(bj.player.hands.is_empty());
    }
    
    /// Ensures correct dealing of cards
    /// - Player given one hand w/ two cards
    /// - Dealer given hand w/ two cards
    #[test]
    fn deal() {
        let mut rng = ChaCha8Rng::seed_from_u64(2);
        let mut deck = MultiDeck::new(6, false);
        deck.shuffle(&mut rng);

        // Set Game Settings
        let settings = GameSettings {
            deck,
            max_splits: 3,
            contains_blank: true,
            init_bet: 10,
            dealer_cutoff: 17,
            dealer_strat: Arc::new(PlayingStrat::Dealer(Box::new(dealer_hard_17))),
            player_strat: Arc::new(PlayingStrat::Player(Box::new(basic_strategy))),
            betting_strat: Arc::new(Box::new(constant_bet)),
            counting_strat: Arc::new(Box::new(update_hi_lo)),
            insurance_strat: Arc::new(Box::new(no_insurance)),
            allow_early_surrender: false,
            allow_late_surrender: false,
            echo: false,
            rng,
        };

        let bet = settings.init_bet.clone();
        let mut bj = Game::from_settings(Arc::new(settings));

        bj.deal(bet);

        assert!(bj.dealer.hand.is_some());
        assert!(bj.get_dealer_upcard().is_some());

        let dealer_cards = bj.dealer.hand.expect("").cards.len();
        let num_player_hands = bj.player.hands.len();
        let num_player_cards = bj.player.hands.first().expect("No hand").cards.len();

        // Asserts
        assert_eq!(dealer_cards, 2);
        assert_eq!(num_player_hands, 1);
        assert_eq!(num_player_cards, 2);


    }
    
    #[test]
    fn test_blank() {

        let mut rng = ChaCha8Rng::seed_from_u64(2);
        let mut deck = MultiDeck::new(6, false);
        deck.shuffle(&mut rng);

        // Set Game Settings
        let settings = GameSettings {
            deck,
            max_splits: 3,
            contains_blank: true,
            init_bet: 10,
            dealer_cutoff: 17,
            dealer_strat: Arc::new(PlayingStrat::Dealer(Box::new(dealer_hard_17))),
            player_strat: Arc::new(PlayingStrat::Player(Box::new(basic_strategy))),
            betting_strat: Arc::new(Box::new(constant_bet)),
            counting_strat: Arc::new(Box::new(update_hi_lo)),
            insurance_strat: Arc::new(Box::new(no_insurance)),
            allow_early_surrender: false,
            allow_late_surrender: false,
            echo: false,
            rng,
        };

        let mut test_pool = GamePool::new(Arc::new(settings));

        test_pool.simulate(100);


    }

    #[test]
    fn run_hand() {
        // Seeded Rng
        let mut rng = ChaCha8Rng::seed_from_u64(2);
        let mut deck = MultiDeck::new(6, false);
        deck.shuffle(&mut rng);

        // Set Game Settings
        let settings = GameSettings {
            deck,
            max_splits: 3,
            contains_blank: true,
            init_bet: 10,
            dealer_cutoff: 17,
            dealer_strat: Arc::new(PlayingStrat::Dealer(Box::new(dealer_hard_17))),
            player_strat: Arc::new(PlayingStrat::Player(Box::new(basic_strategy))),
            betting_strat: Arc::new(Box::new(constant_bet)),
            counting_strat: Arc::new(Box::new(update_hi_lo)),
            insurance_strat: Arc::new(Box::new(no_insurance)),
            allow_early_surrender: false,
            allow_late_surrender: false,
            echo: false,
            rng,
        };

        let bet = settings.init_bet.clone();
        let mut bj = Game::from_settings(Arc::new(settings));

        bj.deal(bet);

        let res: Vec<(game::Winner, EndState)> = bj.play_hand();
        assert_eq!(res.is_empty(), false);
        let player_values: Vec<u8> = bj.player.hands.iter().map(|hand| hand.value()).collect();

        println!("\nRESULTS");
        println!("Dealer FV: {}", bj.dealer.hand.expect("").value());
        println!("Player FV's: {:?}", player_values);
        println!("Winner: {:?}", res);
    }



// |-------------------------|
// |      Decision Tests     |
// |-------------------------|

    /// Tests player split on pairs of Aces and Eights
    #[test]
    fn test_split()  {

        // Aces
        let dealer_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        let player_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        let dealer_hand = Hand::from_cards(dealer_cards, 10, false, false, false);

        let player_hands = vec![Hand::from_cards(player_cards, 10, false, false, false)];

        let expected_decision = PlayerDecision::Split;

        let _ = test_decision(dealer_hand, player_hands, expected_decision);

        // Eights
        let dealer_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        let player_cards = vec![
            Card {
                rank: Rank::Eight,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        let dealer_hand = Hand::from_cards(dealer_cards, 10, false, false, false);

        let player_hands = vec![Hand::from_cards(player_cards, 10, false, false, false)];

        let expected_decision = PlayerDecision::Split;

        test_decision(dealer_hand, player_hands, expected_decision);
    }

    #[test]
    fn test_double_down() {
        // Expected Behavior: Double 
        // Define Cards
        // Ace & Two
        let dealer_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        // Six & Five
        let player_cards = vec![
            Card {
                rank: Rank::Six,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        // Assign to hands
        let dealer_hand = Hand::from_cards(dealer_cards, 10, false, false, false);

        let player_hands = vec![Hand::from_cards(player_cards, 1, false, false, false)];
        
        // Expected Outcome
        let expected_decision = PlayerDecision::Double;

        
        let dealer_strat = Some(Arc::new(PlayingStrat::Dealer(Box::new(dealer_hard_17))));
        let player_strat=  Some(Arc::new(PlayingStrat::Player(Box::new(basic_strategy))));
        let betting_strat: Option<Arc<BettingFunc>> = Some(Arc::new(Box::new(constant_bet)));
            
        // Create Game
        let mut test_game = standard_game(player_strat, dealer_strat, betting_strat, None, None);

        set_hands(&mut test_game, dealer_hand, player_hands.clone());

        let state = test_game.get_state(&player_hands[0]);

        let decision = test_game.player.decide_play(state);

        assert_eq!(decision, expected_decision);

        test_game.double_hand(&player_hands[0]);

        let expected_num_cards = 3;
        assert_eq!(test_game.player.hands[0].cards.len(), expected_num_cards);

        let expected_hand_state = HandState::Finished;
        assert_eq!(test_game.player.hands[0].state, expected_hand_state);

        

    }

    #[test]
    fn test_many() {
        let mut rng = ChaCha8Rng::seed_from_u64(2);
        let mut deck = MultiDeck::new(6, false);
        deck.shuffle(&mut rng);

        // Set Game Settings
        let settings = GameSettings {
            deck,
            max_splits: 3,
            contains_blank: true,
            init_bet: 10,
            dealer_cutoff: 17,
            dealer_strat: Arc::new(PlayingStrat::Dealer(Box::new(dealer_hard_17))),
            player_strat: Arc::new(PlayingStrat::Player(Box::new(basic_strategy))),
            betting_strat: Arc::new(Box::new(constant_bet)),
            counting_strat: Arc::new(Box::new(update_hi_lo)),
            insurance_strat: Arc::new(Box::new(no_insurance)),
            allow_early_surrender: false,
            allow_late_surrender: false,
            echo: false,
            rng,
        };

        let mut test_pool = GamePool::new(Arc::new(settings));

        test_pool.simulate(10000);

        println!("Results: {:?} ", test_pool.results)

    }


// |-------------------------|
// |     Game Result Tests   |
// |-------------------------|

    #[test]
    /// Tests all 3 states of natural blackjack outcomes
    /// 1 - Player Only => Player
    /// 2 - Dealer Only => Dealer
    /// 3 - Both => Tie
    fn test_naturals() {
        // --- Player Natural ---
        // Ace & Six
        let dealer_cards = vec![
            Card {
                rank: Rank::Six,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
                soft: true,
            },
        ];
        
        // Ace & King
        let player_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        let dealer_hand = Hand::from_cards(dealer_cards, 10, false, false, false);

        let player_hands = vec![Hand::from_cards(player_cards, 1, false, false, false)];

        let rng = ChaCha8Rng::seed_from_u64(2);
        let deck = MultiDeck::new(6, false);
        // deck.shuffle(&mut rng);

        // Set Game Settings
        let settings = GameSettings {
            deck,
            max_splits: 3,
            contains_blank: true,
            init_bet: 10,
            dealer_cutoff: 17,
            dealer_strat: Arc::new(PlayingStrat::Dealer(Box::new(dealer_hard_17))),
            player_strat: Arc::new(PlayingStrat::Player(Box::new(basic_strategy))),
            betting_strat: Arc::new(Box::new(constant_bet)),
            counting_strat: Arc::new(Box::new(update_hi_lo)),
            insurance_strat: Arc::new(Box::new(no_insurance)),
            allow_early_surrender: false,
            allow_late_surrender: false,
            echo: false,
            rng,
        };

        let mut test_game = Game::from_settings(Arc::new(settings));

        set_hands(&mut test_game, dealer_hand, player_hands);

        
        let player_natural_outcome = test_game.play_hand();
        let state = player_natural_outcome.first().unwrap().1.clone();
        let expected_winner = Winner::Player;
        let real_winner = player_natural_outcome.first().unwrap().0.clone();
        assert_eq!(expected_winner, real_winner);
        assert!(state.p_natural);
        assert!(!state.d_natural);

        // --- Dealer Natural ---
        test_game.reset_hands();

        // Ace & Jack
        let dealer_cards = vec![
            Card {
                rank: Rank::Jack,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
                soft: true,
            },
        ];
        
        // Ace & Six
        let player_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        let dealer_hand = Hand::from_cards(dealer_cards, 10, false, false, false);

        let player_hands = vec![Hand::from_cards(player_cards, 1, false, false, false)];

        set_hands(&mut test_game, dealer_hand, player_hands);
        let player_natural_outcome = test_game.play_hand();
        let state = player_natural_outcome.first().unwrap().1.clone();
        let expected_winner = Winner::Dealer;
        let real_winner = player_natural_outcome.first().unwrap().0.clone();
        assert_eq!(expected_winner, real_winner);
        assert!(state.d_natural);
        assert!(!state.p_natural);

        // Both Naturals
        test_game.reset_hands();

        // Ace & Jack
        let dealer_cards = vec![
            Card {
                rank: Rank::Jack,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
                soft: true,
            },
        ];
        
        // Ace & Six
        let player_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        let dealer_hand = Hand::from_cards(dealer_cards, 10, false, false, false);

        let player_hands = vec![Hand::from_cards(player_cards, 1, false, false, false)];

        set_hands(&mut test_game, dealer_hand, player_hands);
        let player_natural_outcome = test_game.play_hand();
        let state = player_natural_outcome.first().unwrap().1.clone();
        let expected_winner = Winner::Tie;
        let real_winner = player_natural_outcome.first().unwrap().0.clone();
        assert_eq!(expected_winner, real_winner);
        assert!(state.d_natural);
        assert!(state.p_natural);

    }


// |-------------------------|
// |   Betting Strat Tests   |
// |-------------------------|
    
    #[test]
    /// Ensures correct betting given martingale strat
    fn test_martingale() {

        let mut rng = ChaCha8Rng::seed_from_u64(2);
        let mut deck = MultiDeck::new(6, false);
        deck.shuffle(&mut rng);

        // Set Game Settings
        let settings = GameSettings {
            deck,
            max_splits: 3,
            contains_blank: true,
            init_bet: 10,
            dealer_cutoff: 17,
            dealer_strat: Arc::new(PlayingStrat::Dealer(Box::new(dealer_hard_17))),
            player_strat: Arc::new(PlayingStrat::Player(Box::new(basic_strategy))),
            betting_strat: Arc::new(Box::new(martingale)),
            counting_strat: Arc::new(Box::new(update_hi_lo)),
            insurance_strat: Arc::new(Box::new(no_insurance)),
            allow_early_surrender: false,
            allow_late_surrender: false,
            echo: false,
            rng,
        };
        let settings = Arc::new(settings);
        let mut test_game = Game::from_settings(settings.clone());

        
        // Ace & King
        let dealer_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        // King & King
        let player_cards = vec![
            Card {
                rank: Rank::King,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        let dealer_hand = Hand::from_cards(dealer_cards, 10, false, false, false);
        let player_hands = vec![Hand::from_cards(player_cards, 10, false, false, false)];
        // First Game
        set_hands(&mut test_game, dealer_hand.clone(), player_hands.clone());
        assert_eq!(test_game.init_bet, settings.init_bet); // First game should bet init bet
        let outcome = test_game.play_hand(); 
        assert_eq!(outcome[0].0, Winner::Dealer); // Dealer should win
        assert_eq!(test_game.last_winner, Winner::Dealer);

        // Second Game
        println!("--- SECOND GAME ---");
        assert_eq!(test_game.last_bet, settings.clone().init_bet);
        
        let state = test_game.get_state(&player_hands[0]);
        let bet = (settings.clone().betting_strat)(state);

        let expected_bet = settings.init_bet * 2;
        assert_eq!(bet, expected_bet);
        
    }
    
// |-------------------------|
// |  Insurance Strat Tests  |
// |-------------------------|

    #[test]
    fn test_no_insurance() {
        let insurance_strat:Option<Arc<InsuranceFunc>> = Some(Arc::new(Box::new(no_insurance)));
        let game = standard_game(None, None, None, None, insurance_strat);
        let decision = game.player.decide_insurance(game.get_state(&Hand::new(10)));
        let expected_decision = false;
        assert_eq!(expected_decision, decision);
    }

    fn test() {
        let insurance_strat:Option<Arc<InsuranceFunc>> = Some(Arc::new(Box::new(card_counter_insurance)));
        let game = standard_game(None, None, None, None, insurance_strat);
        let decision = game.player.decide_insurance(game.get_state(&Hand::new(10)));
        let expected_decision = false;
        assert_eq!(expected_decision, decision);
    }



// |-------------------------|
// |   Card Counting Tests   |
// |-------------------------|

    #[test]
    fn test_hi_lo_count() {
        let deck_count = 4;
        let contains_blank = true;
        let mut rng = ChaCha8Rng::seed_from_u64(2);
        let mut deck = MultiDeck::new(deck_count, contains_blank);
        deck.shuffle(&mut rng);

        // Set Game Settings
        let settings = GameSettings {
            deck,
            max_splits: 3,
            contains_blank: true,
            init_bet: 10,
            dealer_cutoff: 17,
            dealer_strat: Arc::new(PlayingStrat::Dealer(Box::new(dealer_hard_17))),
            player_strat: Arc::new(PlayingStrat::Player(Box::new(basic_strategy))),
            betting_strat: Arc::new(Box::new(constant_bet)),
            counting_strat: Arc::new(Box::new(update_hi_lo)),
            insurance_strat: Arc::new(Box::new(no_insurance)),
            allow_early_surrender: false,
            allow_late_surrender: false,
            echo: false,
            rng,
        };
        let settings = Arc::new(settings);
        let mut test_game = Game::from_settings(settings.clone());

        
        // High Cards (-1): Ace & Ten
        let high_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        // Test Counts
        high_cards.iter().for_each(|card| test_game.update_count(card));
        let expected_running = -2;
        let expected_true = expected_running as f64/deck_count as f64;
        assert_eq!(expected_running, test_game.running_count);
        assert_eq!(expected_true, test_game.true_count);
        println!("TRUE: {}", test_game.true_count);
        
        //Low Cards (+1): Two & Six
        let low_cards = vec![
            Card {
                rank: Rank::Two,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        // Test Counts
        low_cards.iter().for_each(|card| test_game.update_count(card));
        let expected_running = 0;
        assert_eq!(expected_running, test_game.running_count);

        // Neutral Cards (+0): 7,8,9
        let neutral_cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        // Test Counts
        neutral_cards.iter().for_each(|card| test_game.update_count(card));
        let expected_running = 0;
        assert_eq!(expected_running, test_game.running_count);
        
    }

    #[test]
    fn test_knock_out_count() {
        let deck_count = 4;
        let contains_blank = true;
        let mut rng = ChaCha8Rng::seed_from_u64(2);
        let mut deck = MultiDeck::new(deck_count, contains_blank);
        deck.shuffle(&mut rng);

        // Set Game Settings
        let settings = GameSettings {
            deck,
            max_splits: 3,
            contains_blank: true,
            init_bet: 10,
            dealer_cutoff: 17,
            dealer_strat: Arc::new(PlayingStrat::Dealer(Box::new(dealer_hard_17))),
            player_strat: Arc::new(PlayingStrat::Player(Box::new(basic_strategy))),
            betting_strat: Arc::new(Box::new(constant_bet)),
            counting_strat: Arc::new(Box::new(update_knock_out)),
            insurance_strat: Arc::new(Box::new(no_insurance)),
            allow_early_surrender: false,
            allow_late_surrender: false,
            echo: false,
            rng,
        };
        let settings = Arc::new(settings);
        let mut test_game = Game::from_settings(settings.clone());

        
        // High Cards (-1): 10..=Ace
        let high_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Queen,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        // Test Counts
        high_cards.iter().for_each(|card| test_game.update_count(card));
        let expected_running = -(high_cards.len() as i32);
        let expected_true = expected_running as f64/deck_count as f64;
        assert_eq!(expected_running, test_game.running_count);
        assert_eq!(expected_true, test_game.true_count);
        
        
        //Low Cards (+1): 2..=7
        let low_cards = vec![
            Card {
                rank: Rank::Two,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        // Test Counts
        low_cards.iter().for_each(|card| test_game.update_count(card));
        let expected_running = expected_running + low_cards.len() as i32;
        assert_eq!(expected_running, test_game.running_count);

        // Neutral Cards (+0): 7,8,9
        let neutral_cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Spades,
                soft: true,
            }
        ];

        // Test Counts
        neutral_cards.iter().for_each(|card| test_game.update_count(card));
        // Running count shouldn't change b/c neutral cards
        assert_eq!(expected_running, test_game.running_count);
    }

    #[test]
    fn test_omega2_count(){
        let deck_count = 4;
        let contains_blank = true;
        let mut rng = ChaCha8Rng::seed_from_u64(2);
        let mut deck = MultiDeck::new(deck_count, contains_blank);
        deck.shuffle(&mut rng);

        // Set Game Settings
        let settings = GameSettings {
            deck,
            max_splits: 3,
            contains_blank: true,
            init_bet: 10,
            dealer_cutoff: 17,
            dealer_strat: Arc::new(PlayingStrat::Dealer(Box::new(dealer_hard_17))),
            player_strat: Arc::new(PlayingStrat::Player(Box::new(basic_strategy))),
            betting_strat: Arc::new(Box::new(constant_bet)),
            counting_strat: Arc::new(Box::new(omega_2)),
            insurance_strat: Arc::new(Box::new(no_insurance)),
            allow_early_surrender: false,
            allow_late_surrender: false,
            echo: false,
            rng,
        };
        let settings = Arc::new(settings);
        let mut test_game = Game::from_settings(settings.clone());

        
        // Very High Cards (-2): 10..=King
        let multiplier = -2;
        let minus_two_cards = vec![
            Card {
                rank: Rank::King,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Queen,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        // Test Counts
        minus_two_cards.iter().for_each(|card| test_game.update_count(card));
        let expected_running =  multiplier * minus_two_cards.len() as i32;
        let expected_true = expected_running as f64/deck_count as f64;
        assert_eq!(expected_running, test_game.running_count);
        assert_eq!(expected_true, test_game.true_count);


        // High Card (-1): 9
        let multiplier = -1;
        let minus_one_cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        // Test Counts
        minus_one_cards.iter().for_each(|card| test_game.update_count(card));
        let expected_running = expected_running + (multiplier * minus_one_cards.len() as i32);
        let expected_true = expected_running as f64/deck_count as f64;
        assert_eq!(expected_running, test_game.running_count);
        assert_eq!(expected_true, test_game.true_count);
        
        
        // Neutral Cards (+0): 8, Ace
        let _multiplier = 0;
        let neutral_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
                soft: true,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Spades,
                soft: true,
            }
        ];

        // Test Counts
        neutral_cards.iter().for_each(|card| test_game.update_count(card));
        // Running/True count shouldn't change b/c neutral cards
        assert_eq!(expected_running, test_game.running_count);
        assert_eq!(expected_true, test_game.true_count);

        //Low Cards (+1): 2,3, 7
        let multiplier = 1;
        let plus_one_cards = vec![
            Card {
                rank: Rank::Two,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        // Test Counts
        plus_one_cards.iter().for_each(|card| test_game.update_count(card));
        let expected_running = expected_running + (multiplier * plus_one_cards.len() as i32);
        let expected_true = expected_running as f64/deck_count as f64;
        assert_eq!(expected_running, test_game.running_count);
        assert_eq!(expected_true, test_game.true_count);

        // +2 : 4, 5, 6
        let multiplier = 2;
        let plus_two_cards = vec![
            Card {
                rank: Rank::Four,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Spades,
                soft: true,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Spades,
                soft: true,
            },
        ];

        // Test Counts
        plus_two_cards.iter().for_each(|card| test_game.update_count(card));
        let expected_running = expected_running + (multiplier * plus_two_cards.len() as i32);
        let expected_true = expected_running as f64/deck_count as f64;
        assert_eq!(expected_running, test_game.running_count);
        assert_eq!(expected_true, test_game.true_count);

    }

}
