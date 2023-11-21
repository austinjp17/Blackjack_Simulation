
use crate::{GameState, deck::{Card, Rank}};

#[derive(Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum PlayerDecision {
    Split,
    Double,
    Hit,
    Stand,
    Surrender,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum DealerUpcardStrength {
    Good,
    Fair,
    Poor
}

pub type StrategyFunc = Box<dyn Fn(GameState) -> PlayerDecision>;

pub enum PlayingStrat
{
    Dealer(StrategyFunc),
    Player(StrategyFunc),
}

/// PLAYING STRATEGY FUNCTIONS

// Dealer never doubles down, splits, or surrenders
// Won't hit above 16 even w/ soft ace
pub fn dealer_strat (game_state: GameState) -> PlayerDecision {
    if game_state.dealer_hand.expect("").value() >= game_state.dealer_cutoff {
        PlayerDecision::Stand
    } else {
        PlayerDecision::Hit
    }
}

pub fn naive_hard (game_state: GameState) -> PlayerDecision {
    if game_state.player_hand.value() >= game_state.dealer_cutoff {
        PlayerDecision::Stand
    } else {
        PlayerDecision::Hit
    }
}

pub fn naive_soft (game_state: GameState) -> PlayerDecision {
    // If at or above cutoff
    if game_state.player_hand.value() >= game_state.dealer_cutoff {

        // If Soft Ace
        if game_state.player_hand.contains_soft_ace() &&
        game_state.player_hand.value() != 21 {
            PlayerDecision::Hit 
        } 
        // Else Stand
        else {
            PlayerDecision::Stand
        }
    } 
    // Hit if below cutoff
    else {
        PlayerDecision::Hit
    }
}

pub fn basic_strategy(game_state: GameState) -> PlayerDecision {
    // Assertions
    assert!(game_state.dealer_upcard.is_some());
    assert!(game_state.dealer_upcard_str.is_some()); 
    

    let player_val = game_state.player_hand.value();
    let dealer_upcard = game_state.dealer_upcard.expect("");
    let dealer_upcard_str = game_state.dealer_upcard_str.expect("");
    
    // Split Check
    // Check for split first b/c we want to split aces not double
    if game_state.player_hand.contains_pair() {
        // Split 8's and Aces
        if game_state.player_hand.contains_pair_of(Card::from_rank(Rank::Eight)) ||  
        game_state.player_hand.contains_pair_of(Card::from_rank(Rank::Ace))
        {
            return PlayerDecision::Split;
        }

        // Generally Split 2's, 3's, & 7's
        if [4, 6, 14_u8].contains(&player_val) {
            // Only split if upcard val not in array
            if ![8, 9, 10, 11].contains(&dealer_upcard.value()) {
                return PlayerDecision::Split;
            }
        }

        // Split 6's if Poor upcard
        if player_val == 12 &&
        dealer_upcard_str == DealerUpcardStrength::Poor {
            return PlayerDecision::Split;
        }
    }

    // Double Down Check
    // One Double down allowed
    // Draw only one more card and turn over
    if !game_state.player_hand.doubled {
        match &player_val {
            11 => { return PlayerDecision::Double; }
            10 => {
                // No double if Ace or 10 upcard
                if  ![10, 11_u8].contains(&dealer_upcard.value()) {
                    return PlayerDecision::Double;
                }
            }
            9 => {
                match &dealer_upcard_str {
                    DealerUpcardStrength::Good => {}
                    _ => { return PlayerDecision::Double } // Double if Fair or Poor
                }
            }

            _ => {}
        }
    }

    // Determing player cutoff to hit based on dealer upcard strength
    let cutoff = match &dealer_upcard_str {
        DealerUpcardStrength::Good => { 17 }
        DealerUpcardStrength::Fair => { 13 }
        DealerUpcardStrength::Poor => { 12 }
    };

    // Hit iff:
    // - If Soft Ace & hand_value above cutoff & not blackjack
    // - Or If below cutoff
    if player_val < cutoff || // Below Cutoff
    game_state.player_hand.contains_soft_ace() &&
    player_val != 21 {  // Not if blackjack 
        PlayerDecision::Hit
    }
    
    // Else Stand (above cutoff w/ no soft ace)
    else {
        PlayerDecision::Stand
    }

}

// Card Counting
// High Cards good for player
// Low cards bad for player reduce chance of dealer bust

pub type CountFunc = Box<dyn Fn(&Card) -> i8>;


pub fn update_hi_lo (card: &Card) -> i8 {
    match card.value() {
        2..=6 => 1, // High
        7..=9 => 0, // Neutral
        10..=11 => -1, // Low
        _ => 0 // Neutral (Never Reached)
    }
}

pub fn update_knock_out(card: &Card) -> i8 {
    match card.value() {
        2..=7 => 1, // High
        8..=9 => 0, // Neutral
        10..=11 => -1, // Low
        _ => 0 // Neutral (Never Reached)
    }
}

pub fn omega_2(card: &Card) -> i8 {
    let plus_two = [4,5,6];
    let plus_one = [2,3,7];
    let zero = [8, 11];
    let minus_one = 9;
    let minus_two = 10;

    if plus_two.contains(&card.value()) { 2 }
    else if plus_one.contains(&card.value()) { 1 }
    else if zero.contains(&card.value()) { 0 }
    else if card.value() == minus_one { -1 }
    else if card.value() == minus_two { -2 }
    else { 0 } // Never Reached

}


