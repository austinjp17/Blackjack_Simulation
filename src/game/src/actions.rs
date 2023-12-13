use rand::Rng;
use crate::{Game, Hand, HandState, deck::{Card, MultiDeck}, playing_strategy::StratReturn, GameState};
use std::{time::Instant, ops::Div};


impl <R: Rng + Clone> Game <R> {

    pub fn deal(&mut self, init_bet: u32) {
        // Check if player hands vec empty
        if self.player.hands.is_empty() {
            self.player.hands.push(Hand::new(init_bet))
        }

        // Check if player 

        // Deal Player Hands
        let mut temp_hands = std::mem::take(&mut self.player.hands);
        for hand in temp_hands.iter_mut() {
            hand.set_state(HandState::Playing);
            while hand.cards.len() < 2 {
                hand.cards.push(self.draw());
            }
        }
        // Assign player hand
        self.player.hands = temp_hands;

        // Deal Dealer
        if self.dealer.hand.is_none() {
            self.dealer.hand = Some(Hand::new(init_bet))
        }
        else {
            self.dealer.hand.as_mut().expect("None handled").init_bet = init_bet;
        }
        self.dealer.hand.as_mut().expect("").set_state(HandState::Playing);
        while self.dealer.hand.as_ref().expect("").cards.len() < 2 {
            let draw = self.draw();
            self.dealer.hand.as_mut().expect("").cards.push(draw);
        }
        
        
    }

    pub fn debug_deal(&mut self, init_bet: u32) {
        let start = Instant::now();
    
        // Check if player hands vec empty
        if self.player.hands.is_empty() {
            self.player.hands.push(Hand::new(init_bet))
        }
    
        let check_hands_time = start.elapsed();
        let deal_start = Instant::now();
        println!("Time to check and update player hands: {:?}", check_hands_time);
    
        // Deal Player Hands
        let mut temp_hands = std::mem::take(&mut self.player.hands);
        
        let loop_start = Instant::now();
        for hand in temp_hands.iter_mut() {
            hand.set_state(HandState::Playing);
            while hand.cards.len() < 2 {
                let draw_start = Instant::now();
                hand.cards.push(self.draw());
                let draw_time = draw_start.elapsed();
                println!("Time to draw: {:?}", draw_time);
            }
        }
        let loop_time = loop_start.elapsed();
        println!("Time Loop: {:?}", loop_time);

    
        let player_deal_time = deal_start.elapsed();
        let dealer_deal_start = Instant::now();
    
        // Deal Dealer
        if self.dealer.hand.is_none() {
            self.dealer.hand = Some(Hand::new(init_bet))
        }
        self.dealer.hand.as_mut().expect("").set_state(HandState::Playing);
        while self.dealer.hand.as_ref().expect("").cards.len() < 2 {
            let draw = self.draw();
            self.dealer.hand.as_mut().expect("").cards.push(draw);
        }
    
        let dealer_deal_time = dealer_deal_start.elapsed();
        let total_time = start.elapsed();
    
        
        println!("Time to deal to player: {:?}", player_deal_time);
        println!("Time to deal to dealer: {:?}", dealer_deal_time);
        println!("Total deal function time: {:?}", total_time);
    
        // Restoring the player hands
        self.player.hands = temp_hands;

    }

    pub fn reset_hands(&mut self) {
        self.player.hands = vec![];
        self.dealer.hand = None
    }

    pub fn new_deck(&mut self) {
        // Empty Played Cards
        self.played_cards.clear();

        // New deck using existint settings
        let new_deck = MultiDeck::new(self.deck.deck_count, self.deck.contains_blank);

        // Assign new deck and shuffle
        self.deck = new_deck;
        self.deck.shuffle(&mut self.rng);

        // Insert blank if present
        if self.deck.contains_blank {
            self.deck.insert_blank(&mut self.rng);
        }
    }
    
    pub fn draw(&mut self) -> Card { 
        // If cards in deck
        let new_card = self.deck.draw();

        // Debug Statement
        if self.echo {
            if new_card.is_some() {
                println!("Card Draw: {}", &new_card.unwrap());
            } else {
                println!("Card Draw: {:?}", &new_card)
            }
        }
        
        // Give card back if exists, or refresh deck and redraw
        match new_card {
            Some(card) => {
                if !card.is_blank() {
                    self.played_cards.push(card);
                    self.update_count(self.get_state(None));
                    
                    
                    return card;
                }
            },
            None => {
                // Else new deck
                let expected_deck_size = match self.deck.contains_blank {
                    true => {(52*self.deck.deck_count as usize) + 1}, // Add card for blank
                    false => {52*self.deck.deck_count as usize}
                };
                
                self.new_deck();
                assert_eq!(expected_deck_size, self.deck.decks.cards.len());

                // Reset Counts && Played Cards
                self.running_count = 0;
                self.true_count = 0.0;
                self.played_cards = vec![];

                // Debug Statement
                if self.echo {
                    println!("Creating New Deck");
                    println!("Expected: {}", 52*self.deck.deck_count as usize);
                    println!("New Deck size: {}", self.deck.decks.cards.len());
                }
            }
        }
        
        // Draw card from new deck
        let new_card = self.deck.draw();
        assert!(new_card.is_some()); 
        let new_card = new_card.unwrap();

        // Update Played Cards
        self.played_cards.push(new_card);
        // Update Count
        self.update_count(self.get_state(None));
        
        new_card
        
    }

    pub fn update_count(&mut self, state: GameState) {
        let delta = match self.player.counting_strat.get_decision(state) {
            StratReturn::Count(d) => d,
            _ => unreachable!("Always Count")
        };
        self.running_count += delta as i32;
        self.true_count = (self.running_count as f64).div(self.deck.deck_count as f64)
    }

    pub fn hit_dealer(&mut self) {
        assert!(self.dealer.hand.is_some()); // Dealer must have hand
        let new_card = self.draw(); // Draw Card

        // Get Dealer Hand Mut Ref
        let dealer_hand = self.dealer.hand.as_mut().expect("");
        dealer_hand.cards.push(new_card); // Add card to dealer's hand
        
        // Soft Ace Check
        // If Soft Ace && over 21 w/ the new draw => Deflate Ace
        if dealer_hand.value() > 21 && dealer_hand.contains_soft_ace() {
            // Debug Statement
            if self.echo {
                println!("Deflate Ace");
            }
            dealer_hand.deflate_ace();
        }
    }

    pub fn hit_player(&mut self, target_hand: &Hand) {
        // Draw card
        let draw = self.draw();
        // Find target hand
        for hand in self.player.hands.iter_mut() {
            if target_hand == hand {
                // Add card to hand
                if self.echo {
                    println!("Draw: {}", &draw);
                }
                
                hand.cards.push(draw);

                // Soft Ace Check
                if hand.value() > 21 {
                    if hand.contains_soft_ace() {
                        if self.echo {
                            println!("Deflate");
                        }
                        hand.deflate_ace();
                    }
                    else {
                        break;
                    }
                }
                break;
            }
        }
    }

    pub fn split_hand(&mut self, target_hand: &Hand) {
        let x_card;
        let mut x_hand;
        let y_card;
        let mut y_hand;

        // Find target hand
        for (i, hand) in self.player.hands.iter_mut().enumerate() {
            if target_hand == hand {
                // Clone target (2 card hand)
                let mut target = hand.clone();

                // Pop Card from hand and add to new hand
                // Draw second card for hand
                x_card = target.cards.pop().unwrap();
                x_hand = Hand::from_cards(vec![x_card], target.init_bet, false, false, true);
                x_hand.cards.push(self.draw());
                
                // Pop second card and add to new hand
                // Draw second card for hand
                y_card = target.cards.pop().unwrap();
                y_hand = Hand::from_cards(vec![y_card], target.init_bet, false, false, true);
                y_hand.cards.push(self.draw());

                // Add hands to player
                self.player.hands.push(x_hand);
                self.player.hands.push(y_hand);

                // Remove original hand from player
                self.player.hands.remove(i);

                break;
            }
        }
        
        

    }

    pub fn double_hand(&mut self, target_hand: &Hand) {
        let single_draw = self.draw();
        for hand in self.player.hands.iter_mut() {
            if target_hand == hand {
                hand.doubled = true;
                hand.cards.push(single_draw);
                hand.state = HandState::Finished;
                break;
            }
        }
    }

    pub fn player_natural(&mut self, target_hand: &Hand) {
        for hand in self.player.hands.iter_mut() {
            if target_hand == hand {
                hand.natural = true;
                break;
            }
        }
    }

    pub fn stand_player(&mut self, target_hand: &Hand) {
        for hand in self.player.hands.iter_mut() {
            if target_hand == hand {
                hand.set_state(HandState::Finished);
            }
        }
    }

    pub fn surrender(&mut self, target_hand: &Hand, state: HandState) {
        for hand in self.player.hands.iter_mut() {
            if target_hand == hand {
                hand.set_state(state);
                break;
            }
        }
    }

    // Test Functions

    pub fn set_player_hands(&mut self, new_hands: Vec<Hand>) {
        self.player.hands = new_hands;
    }

    pub fn set_dealer_hand(&mut self, new_hand: Hand) {
        self.dealer.hand = Some(new_hand);
    }
}