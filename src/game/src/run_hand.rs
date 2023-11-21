use crate::{deck::Hand, playing_strategy::PlayerDecision, EndState, Game, HandState, Winner};
use rand::Rng;

impl<R: Rng + Clone> Game<R> {
    pub fn natural_check(&self, hand: &Hand) -> bool {
        if hand.cards.len() == 2 && !hand.split_child && hand.value() == 21 {
            return true;
        }
        false
    }

    pub fn handle_player_hand(&mut self, hand: &Hand) {
        if !hand.is_finished() {
            // Check for Natural on first iteration
            if self.natural_check(hand) {
                self.player_natural(hand);
            }

            // Player hand response
            let decision = self.player.decide_play(self.get_state(hand));
            if self.echo {
                println!("\n!! New Hand !!\nBet: {}", &hand.init_bet);
                println!("\n___PLAYER___");
                println!("Current Hand: {}", hand);
                println!("Value: {}", hand.value());
                println!("Decision: {:?}", decision);
            }
            match decision {
                PlayerDecision::Stand => self.stand_player(hand),
                PlayerDecision::Hit => self.hit_player(hand),
                PlayerDecision::Split => self.split_hand(hand),
                PlayerDecision::Double => self.double_hand(hand),
                PlayerDecision::Surrender => self.surrender(hand),
            }
        }
    }

    pub fn play_hand(&mut self) -> Vec<(Winner, EndState)> {
        // Set Game Bet
        assert!(self.dealer.hand.is_some());
        // Dealer always has one hand and the initial bet if variable due to player
        self.last_bet = self.dealer.hand.as_ref().expect("Handled").init_bet;

        // Player: Always First
        loop {
            if self.player.is_finished() {
                break;
            }

            // Player may have multiple hands if split
            let hands = self.player.hands.clone();
            for hand in hands.iter() {
                self.handle_player_hand(hand)
            }
        }

        // Dealer Play
        if self.echo {
            println!("\n ___DEALER___");
            println!("Upcard: {:?}", self.get_dealer_upcard());
            println!("Cutoff: {}", self.dealer.cutoff);
        }
        loop {
            if self.dealer.is_finished() {
                break;
            }

            // Check for Natural on first iteration
            if self
                .dealer
                .hand
                .as_ref()
                .expect("No Dealer Cards")
                .cards
                .len()
                == 2
                && !self.dealer.hand.as_ref().expect("").split_child
                && self.dealer.hand.as_ref().expect("").value() == 21
            {
                self.dealer.hand.as_mut().expect("").natural = true;
            }

            let decision = self
                .dealer
                .decide_play(self.get_state(self.dealer.hand.as_ref().expect("")));

            if self.echo {
                println!("\nCurrent Hand: {}", self.dealer.hand.as_ref().expect(""));
                println!("Value: {}", &self.dealer.hand.as_ref().expect("").value());
                println!("Decision: {:?}", decision);
            }

            match decision {
                PlayerDecision::Hit => self.hit_dealer(),
                PlayerDecision::Stand => {
                    self.dealer
                        .hand
                        .as_mut()
                        .expect("")
                        .set_state(HandState::Finished);
                }
                _ => {} // Dealer can only hit or stand
            }
        }

        // Determine winner
        let hand_results: Vec<(Winner, EndState)> = self
            .player
            .hands
            .iter()
            .map(|hand| {
                // Var initialization
                let mut winner: Option<Winner> = None;
                let mut end_state = EndState::default();
                let dealer_val = self.dealer.hand.as_ref().expect("").value();

                // Assign State flag
                end_state.hand_bet = self.last_bet;

                if hand.doubled {
                    end_state.p_doubled = true
                }

                if hand.natural {
                    end_state.p_natural = true
                }

                if self.dealer.hand.as_ref().expect("").natural {
                    end_state.d_natural = true
                }

                // Bust Checks
                // Player would bust before dealer, so check first and award dealer win even if
                // they go over 21 drawing, b/c wouldn't draw any in real life.

                // Player Bust
                if hand.value() > 21 {
                    end_state.p_bust = true;
                    winner = Some(Winner::Dealer)
                }

                // Dealer Bust
                if dealer_val > 21 {
                    end_state.d_bust = true;
                    if winner.is_none() {
                        winner = Some(Winner::Player)
                    }
                }

                // Tie
                if hand.value() == dealer_val {
                    winner = Some(Winner::Tie)
                }
                // Player Win
                else if hand.value() > dealer_val {
                    if winner.is_none() {
                        winner = Some(Winner::Player);
                    }
                }
                // Dealer Win
                else {
                    if winner.is_none() {
                        winner = Some(Winner::Dealer);
                    }
                }

                (winner.expect("No Winner Found"), end_state)
            })
            .collect();

        // __SET LAST WINNER__
        let mut last_winner = Winner::None;

        match hand_results.len() {
            0 => {} // No Results (Shouldn't be reached)
            // If no split (1 hand only)
            1 => {
                last_winner = hand_results.first().unwrap().0.clone();
            }

            _ => {
                // If Split (Multiple hands)
                // Whoever has most wins is given 'last_winner'
                // If even wins, Tie returned
                let mut num_player_wins: u8 = 0;
                let mut num_dealer_wins: u8 = 0;
                let mut num_ties: u8 = 0;

                hand_results.iter().for_each(|(winner, _)| match winner {
                    Winner::Player => {
                        num_player_wins += 1;
                    }
                    Winner::Dealer => {
                        num_dealer_wins += 1;
                    }
                    Winner::Tie => num_ties += 1,
                    Winner::None => {}
                });

                // Set Last Winner
                // Player wins majority
                if num_player_wins > num_dealer_wins {
                    last_winner = Winner::Player
                }
                // Tie
                else if num_player_wins == num_dealer_wins {
                    last_winner = Winner::Tie
                }
                // Dealer win majority
                else {
                    last_winner = Winner::Dealer
                }
            }
        }

        // Assign last winner & return hand results
        self.last_winner = last_winner;
        hand_results
    }
}
