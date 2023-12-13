use crate::{
    deck::{Card, Rank},
    GameState,
};

#[derive(Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum PlayerDecision {
    Split,
    Double,
    Hit,
    Stand,
    EarlySurrender,
    LateSurrender,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum DealerUpcardStrength {
    Good,
    Fair,
    Poor,
}


pub enum StratReturn {
    Play(PlayerDecision),
    Bet(u32),
    Count(i8),
    Insurance(bool)
}
pub trait StrategyFunc {
    fn get_decision(&self, state: GameState) -> StratReturn;
    fn to_string(&self) -> String;
}

/// PLAYING STRATEGY FUNCTIONS

// Dealer Strat
// Dealer never doubles down, splits, or surrenders
// Won't hit above cutoff even w/ soft ace

pub struct DealerPlay;
impl StrategyFunc for DealerPlay {
    fn get_decision(&self, state: GameState) -> StratReturn {
        let dealer_hand_value = state.dealer_hand.expect("").value();
        if dealer_hand_value >= state.dealer_cutoff {
            StratReturn::Play(PlayerDecision::Stand)
        } else {
            StratReturn::Play(PlayerDecision::Hit)
        }
    }

    fn to_string(&self) -> String {
        "Dealer Play".to_string()
    }
}

// -- Player Strats --

pub struct MimicDealer;
impl StrategyFunc for MimicDealer {
    fn get_decision(&self, state: GameState) -> StratReturn {
        if state.player_hand.value() >= state.dealer_cutoff {
            StratReturn::Play(PlayerDecision::Stand)
        } else {
            StratReturn::Play(PlayerDecision::Hit)
        }
    }

    fn to_string(&self) -> String {
        "Mimic Dealer".to_string()
    }
}

pub struct NaiveSoft;
impl StrategyFunc for NaiveSoft {
    fn get_decision(&self, state: GameState) -> StratReturn {
        // If at or above cutoff
        if state.player_hand.value() >= state.dealer_cutoff {
            // If Soft Ace
            if state.player_hand.contains_soft_ace() && state.player_hand.value() < 18 {
                StratReturn::Play(PlayerDecision::Hit)
            }
            // Else Stand
            else {
                StratReturn::Play(PlayerDecision::Stand)
            }
        }
        // Hit if below cutoff
        else {
            StratReturn::Play(PlayerDecision::Hit)
        }
    }

    fn to_string(&self) -> String {
        "Naive Soft".to_string()
    }
}

// Player uses dealer upcard strength to set a 'stand' cutoff value.
// No other playing options considered
pub struct CutoffOnly;
impl StrategyFunc for CutoffOnly {
    fn get_decision(&self, state: GameState) -> StratReturn {
        let dealer_upcard_str = state.dealer_upcard_str.expect("");
        let player_cutoff = match &dealer_upcard_str {
            DealerUpcardStrength::Good => 17,
            DealerUpcardStrength::Fair => 13,
            DealerUpcardStrength::Poor => 12,
        };

        let player_val = state.player_hand.value();
        // Hit iff:
        // - If Soft Ace & hand_value above cutoff & not blackjack
        // - Or If below cutoff
        if player_val < player_cutoff || // Below Cutoff
        state.player_hand.contains_soft_ace() &&
        player_val != 21
        {
            // Not if blackjack
            StratReturn::Play(PlayerDecision::Hit)
        }
        // Else Stand (above cutoff w/ no soft ace)
        else {
            StratReturn::Play(PlayerDecision::Stand)
        }
    }

    fn to_string(&self) -> String {
        "Cutoff Only".to_string()
    }
}


pub struct DoubleOnly;
impl StrategyFunc for DoubleOnly {
    fn get_decision(&self, state: GameState) -> StratReturn {
        let player_val = state.player_hand.value();
        let dealer_upcard = state.dealer_upcard.expect("");
        let dealer_upcard_str = state.dealer_upcard_str.expect("");
        // Double Check
        if !state.player_hand.doubled {
            match &player_val {
                11 => {
                    return StratReturn::Play(PlayerDecision::Double);
                }
                10 => {
                    // No double if Ace or 10 upcard
                    if ![10, 11_u8].contains(&dealer_upcard.value()) {
                        return StratReturn::Play(PlayerDecision::Double);
                    }
                }
                9 => {
                    match &dealer_upcard_str {
                        DealerUpcardStrength::Good => {}
                        _ => return StratReturn::Play(PlayerDecision::Double), // Double if Fair or Poor
                    }
                }

                _ => {}
            }
        }

        // Else default behavior
        NaiveSoft.get_decision(state)
    }

    fn to_string(&self) -> String {
        "Double Only".to_string()
    }
}

pub struct SplitOnly;
impl StrategyFunc for SplitOnly {
    fn get_decision(&self, state: GameState) -> StratReturn {
        let player_val = state.player_hand.value();
        let dealer_upcard = state.dealer_upcard.expect("");
        let dealer_upcard_str = state.dealer_upcard_str.expect("");

        if state.player_hand.contains_pair() {
            // Split 8's and Aces
            if state.player_hand
                .contains_pair_of(Card::from_rank(Rank::Eight))
                || state
                    .player_hand
                    .contains_pair_of(Card::from_rank(Rank::Ace))
            {
                return StratReturn::Play(PlayerDecision::Split);
            }

            // Generally Split 2's, 3's, & 7's
            if [4, 6, 14_u8].contains(&player_val) {
                // Only split if upcard val not in array
                if ![8, 9, 10, 11].contains(&dealer_upcard.value()) {
                    return StratReturn::Play(PlayerDecision::Split);
                }
            }

            // Split 6's if Poor upcard
            if player_val == 12 && dealer_upcard_str == DealerUpcardStrength::Poor {
                return StratReturn::Play(PlayerDecision::Split);
            }
        }
        // Default behavior if no split
        NaiveSoft.get_decision(state)
    }

    fn to_string(&self) -> String {
        "Split Only".to_string()
    }
}

pub struct BasicStrategy;
impl StrategyFunc for BasicStrategy {
    fn get_decision(&self, state: GameState) -> StratReturn {
        // Assertions
        assert!(state.dealer_upcard.is_some());
        assert!(state.dealer_upcard_str.is_some());

        let player_val = state.player_hand.value();
        let dealer_upcard = state.dealer_upcard.expect("");
        let dealer_upcard_str = state.dealer_upcard_str.expect("");

        // Early Surrender Check

        // Split Check
        // Check for split first b/c we want to split aces not double
        if state.player_hand.contains_pair() {
            // Split 8's and Aces
            if state
                .player_hand
                .contains_pair_of(Card::from_rank(Rank::Eight))
                || state
                    .player_hand
                    .contains_pair_of(Card::from_rank(Rank::Ace))
            {
                return StratReturn::Play(PlayerDecision::Split);
            }

            // Generally Split 2's, 3's, & 7's
            if [4, 6, 14_u8].contains(&player_val) {
                // Only split if upcard val not in array
                if ![8, 9, 10, 11].contains(&dealer_upcard.value()) {
                    return StratReturn::Play(PlayerDecision::Split);
                }
            }

            // Split 6's if Poor upcard
            if player_val == 12 && dealer_upcard_str == DealerUpcardStrength::Poor {
                return StratReturn::Play(PlayerDecision::Split);
            }
        }

        // Double Down Check
        // One Double down allowed
        // Draw only one more card and turn over
        if !state.player_hand.doubled {
            match &player_val {
                11 => {
                    return StratReturn::Play(PlayerDecision::Double);
                }
                10 => {
                    // No double if Ace or 10 upcard
                    if ![10, 11_u8].contains(&dealer_upcard.value()) {
                        return StratReturn::Play(PlayerDecision::Double);
                    }
                }
                9 => {
                    match &dealer_upcard_str {
                        DealerUpcardStrength::Good => {}
                        _ => return StratReturn::Play(PlayerDecision::Double), // Double if Fair or Poor
                    }
                }

                _ => {}
            }
        }

        // Determing player cutoff to hit based on dealer upcard strength
        let cutoff = match &dealer_upcard_str {
            DealerUpcardStrength::Good => 17,
            DealerUpcardStrength::Fair => 13,
            DealerUpcardStrength::Poor => 12,
        };

        // Hit iff:
        // - If Soft Ace & hand_value above cutoff & not blackjack
        // - Or If below cutoff
        if player_val < cutoff || // Below Cutoff
        state.player_hand.contains_soft_ace() &&
        player_val < 18
        {
            // Not if blackjack
            StratReturn::Play(PlayerDecision::Hit)
        }
        // Else Stand (above cutoff w/ no soft ace)
        else {
            StratReturn::Play(PlayerDecision::Stand)
        }
    }

    fn to_string(&self) -> String {
        "Basic Strategy".to_string()
    }
}


// |-------------------------|
// |  CARD COUNTING STRATS   |
// |-------------------------|

// High Cards good for player
// Low cards bad for player reduce chance of dealer bust

pub type CountFunc = Box<dyn Fn(&Card) -> i8>;

// Neg count means lower number of 10 value cards

pub fn update_hi_lo(card: &Card) -> i8 {
    match card.value() {
        2..=6 => 1,    // High
        7..=9 => 0,    // Neutral
        10..=11 => -1, // Low
        _ => 0,        // Neutral (Never Reached)
    }
}

pub fn update_knock_out(card: &Card) -> i8 {
    match card.value() {
        2..=7 => 1,    // High
        8..=9 => 0,    // Neutral
        10..=11 => -1, // Low
        _ => 0,        // Neutral (Never Reached)
    }
}

pub fn omega_2(card: &Card) -> i8 {
    let plus_two = [4, 5, 6];
    let plus_one = [2, 3, 7];
    let zero = [8, 11];
    let minus_one = 9;
    let minus_two = 10;

    if plus_two.contains(&card.value()) {
        2
    } else if plus_one.contains(&card.value()) {
        1
    } else if zero.contains(&card.value()) {
        0
    } else if card.value() == minus_one {
        -1
    } else if card.value() == minus_two {
        -2
    } else {
        0
    } // Never Reached
}

// |-------------------------|
// |  INSURANCE STRATEGIES   |
// |-------------------------|

/// Offered on dealer ace upcard before hole card looked at
/// 2:1 payout
/// Max bet of half init bet
/// Paid out if dealer natural
pub type InsuranceFunc = Box<dyn Fn(GameState) -> bool>;

pub struct NoInsurance;
impl StrategyFunc for NoInsurance {
    fn get_decision(&self, _: GameState) -> StratReturn { StratReturn::Insurance(false) }

    fn to_string(&self) -> String { "No Insurance".to_string() }
}


// pub fn card_counter_insurance(_state: GameState) -> bool {
//     false
// }
