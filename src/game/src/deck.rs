#![allow(dead_code)]
//! Provides a [`Deck`] implementation for simulating black jack games.

use crate::playing_strategy::DealerUpcardStrength;
use core::slice::Iter;
use rand::seq::SliceRandom; // Required for shuffling the deck
use rand::Rng;
use std::fmt;

/// Card Suit representation
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub fn iterator() -> Iter<'static, Suit> {
        static SUITS: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        SUITS.iter()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Clubs => write!(f, "Clubs"),
            Suit::Spades => write!(f, "Spades"),
        }
    }
}

/// Card rank representation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Rank {
    Ace = 1, // or 11 depending on if the card is "inflated", the default state.
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Blank = u8::MAX,
}

impl Rank {
    pub fn value(&self, soft: bool) -> u8 {
        match *self {
            Rank::Ace => match soft {
                true => 11,
                false => 1,
            },
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4u8,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Blank => 0,
        }
    }

    pub fn iterator() -> Iter<'static, Rank> {
        static RANKS: [Rank; 13] = [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];
        RANKS.iter()
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::Ace => write!(f, "Ace"),
            Rank::Two => write!(f, "Two"),
            Rank::Three => write!(f, "Three"),
            Rank::Four => write!(f, "Four"),
            Rank::Five => write!(f, "Five"),
            Rank::Six => write!(f, "Six"),
            Rank::Seven => write!(f, "Seven"),
            Rank::Eight => write!(f, "Eight"),
            Rank::Nine => write!(f, "Nine"),
            Rank::Ten => write!(f, "Ten"),
            Rank::Jack => write!(f, "Jack"),
            Rank::Queen => write!(f, "Queen"),
            Rank::King => write!(f, "King"),
            Rank::Blank => write!(f, "Blank"),
        }
    }
}

// Define a card as a combination of rank and suit
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub soft: bool,
}

impl Card {
    pub fn from_rank(rank: Rank) -> Self {
        Card {
            rank,
            suit: Suit::Hearts,
            soft: true,
        }
    }

    pub fn is_blank(&self) -> bool {
        self.rank == Rank::Blank
    }

    pub fn is_inflated(&self) -> bool {
        self.soft
    }

    pub fn deflate(&mut self) {
        self.soft = false;
    }

    pub fn inflate(&mut self) {
        self.soft = true;
    }

    pub fn value(&self) -> u8 {
        self.rank.value(self.soft)
    }

    pub fn get_dealer_str(&self) -> DealerUpcardStrength {
        match self.value() {
            1 => DealerUpcardStrength::Good,
            7..=11 => DealerUpcardStrength::Good,
            2..=3 => DealerUpcardStrength::Fair,
            4..=6 => DealerUpcardStrength::Poor,

            _ => DealerUpcardStrength::Good, // Never reached, card value bounded [1-11] inclusively
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit_code = match self.suit {
            Suit::Hearts => '\u{2665}',
            Suit::Diamonds => '\u{2666}',
            Suit::Clubs => '\u{2663}',
            Suit::Spades => '\u{2660}', //127184
        };

        write!(f, "[{} {}]", self.value(), suit_code)
    }
}

// Define a deck
#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    // Constructs a new, sorted standard deck
    pub fn standard() -> Self {
        let mut cards = Vec::with_capacity(52);

        for &suit in Suit::iterator() {
            for &rank in Rank::iterator() {
                cards.push(Card {
                    rank,
                    suit,
                    soft: true,
                });
            }
        }
        Self { cards }
    }

    // Shuffles the deck in place
    pub fn shuffle_rng<R>(&mut self, rng: &mut R)
    where
        R: Rng + ?Sized,
    {
        self.cards.shuffle(rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        // If deck out of cards
        self.cards.pop()
    }
}

#[derive(Debug, Clone)]
pub struct MultiDeck {
    pub decks: Deck,
    pub deck_count: u8,
    pub contains_blank: bool,
}

impl MultiDeck {
    pub fn new(size: u8, contains_blank: bool) -> Self {
        let mut decks_split = Vec::with_capacity(size as usize);
        for _ in 0..size {
            decks_split.push(Deck::standard());
        }

        let flattened_decks = decks_split.iter().flat_map(|deck| deck.cards.clone()).collect::<Vec<Card>>();

        assert_eq!(flattened_decks.len(), 52*size as usize);

        let decks = Deck { cards: flattened_decks };

        Self { decks, contains_blank, deck_count: size }
    }

    pub fn insert_blank(&mut self, rng: &mut impl Rng) {
        // Last 60-70 cards randomly
        let blank_card = Card {
            rank: Rank::Blank,
            suit: Suit::Hearts,
            soft: true,
        };
        let offset = rng.gen_range(59..70); // Blank card put randomly 
        // let index = self.decks.cards.len() - offset;
        self.decks.cards.insert(offset, blank_card);
    }

    // Will always return card if blank card included
    // can be none if blank excluded
    pub fn draw(&mut self) -> Option<Card> {
        if let Some(card) = self.decks.draw() { return Some(card); }
        None
    }

    pub fn shuffle(&mut self, rng: &mut impl Rng) {
        self.decks.shuffle_rng(rng);
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum HandState {
    Init,        // Not dealt cards yet
    Playing,     // Active player
    EarlySurrender, // Forfeit: Half bet returned
    LateSurrender,
    Finished,    // Standing player
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Hand {
    pub cards: Vec<Card>,  // Current Cards
    pub state: HandState,  // State of Hand -- ::new() -> State::Init
    pub init_bet: u32,     // Inital Bet
    pub doubled: bool,     // Doubled Down Flag
    pub natural: bool,     // Natural BlackJack Flag
    pub split_child: bool, // Derivative Hand Flag -- Indicates whether hand resulted from split
}

impl Default for Hand {
    fn default() -> Self {
        Hand {
            cards: vec![],
            state: HandState::Init,
            init_bet: 10, // State Obj's
            doubled: false,
            natural: false,
            split_child: false, // Flags
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in self.cards.iter() {
            let _ = write!(f, "{}", card);
        }
        write!(f, "")
    }
}

impl Hand {
    pub fn new(init_bet: u32) -> Self {
        Hand {
            cards: vec![],
            state: HandState::Init,
            init_bet,
            ..Default::default()
        }
    }

    pub fn from_cards(
        cards: Vec<Card>,
        init_bet: u32,
        doubled: bool,
        natural: bool,
        split_child: bool,
    ) -> Self {
        Hand {
            cards,
            state: HandState::Playing,
            init_bet,
            doubled,
            natural,
            split_child,
        }
    }

    pub fn contains_pair(&self) -> bool {
        self.cards.get(0).expect("No cards").rank == self.cards.get(1).expect("No 2nd card").rank
    }

    pub fn contains_pair_of(&self, target_card: Card) -> bool {
        if self.contains_pair() {
            self.cards.get(0).expect("No cards").rank == target_card.rank
        } else {
            false
        }
    }

    pub fn contains_soft_ace(&self) -> bool {
        self.cards.iter().any(|card| {
            if card.value() == 11 {
                card.is_inflated()
            } else {
                false
            }
        })
    }

    pub fn deflate_ace(&mut self) {
        for card in self.cards.iter_mut() {
            // Find Ace
            if card.value() == 11 {
                // Check for soft ace
                if card.is_inflated() {
                    card.deflate()
                }
            }
        }
    }

    pub fn value(&self) -> u8 {
        self.cards.iter().map(|card| card.value()).sum()
    }

    pub fn set_state(&mut self, new_state: HandState) { self.state = new_state }

    pub fn is_surrendered(&self) -> bool 
    { self.state == HandState::EarlySurrender || self.state == HandState::LateSurrender }

    pub fn is_finished(&self) -> bool { self.state == HandState::Finished || self.is_surrendered() }
}

/// TEST
#[cfg(test)]
mod tests {
    // use crate::{Card, Deck, Rank, Suit, Hand};

    // #[test]
    // fn test_standard_deck_gen() {
    //     let deck = Deck::standard();
    //     assert_eq!(deck.cards.len(), 52);

    //     let mut expected_cards = Vec::new();
    //     for &suit in Suit::iterator() {
    //         for &rank in Rank::iterator() {
    //             expected_cards.push(Card {
    //                 rank,
    //                 suit,
    //                 inflated: true,
    //             });
    //         }
    //     }

    //     for (i, card) in deck.cards.iter().enumerate() {
    //         assert_eq!(card.to_string(), expected_cards[i].to_string());
    //     }
    // }

    // #[test]
    // fn test_standard_deck_static() {
    //     let deck = Deck::standard();
    //     let mut deck_as_strings = deck
    //         .cards
    //         .iter()
    //         .map(|c| format!("{c}"))
    //         .collect::<Vec<String>>();
    //     assert_eq!(deck_as_strings.len(), 52);

    //     let mut expected_cards = vec![
    //         "Ace of Clubs",
    //         "Two of Clubs",
    //         "Three of Clubs",
    //         "Four of Clubs",
    //         "Five of Clubs",
    //         "Six of Clubs",
    //         "Seven of Clubs",
    //         "Eight of Clubs",
    //         "Nine of Clubs",
    //         "Ten of Clubs",
    //         "Jack of Clubs",
    //         "Queen of Clubs",
    //         "King of Clubs",
    //         "Ace of Diamonds",
    //         "Two of Diamonds",
    //         "Three of Diamonds",
    //         "Four of Diamonds",
    //         "Five of Diamonds",
    //         "Six of Diamonds",
    //         "Seven of Diamonds",
    //         "Eight of Diamonds",
    //         "Nine of Diamonds",
    //         "Ten of Diamonds",
    //         "Jack of Diamonds",
    //         "Queen of Diamonds",
    //         "King of Diamonds",
    //         "Ace of Hearts",
    //         "Two of Hearts",
    //         "Three of Hearts",
    //         "Four of Hearts",
    //         "Five of Hearts",
    //         "Six of Hearts",
    //         "Seven of Hearts",
    //         "Eight of Hearts",
    //         "Nine of Hearts",
    //         "Ten of Hearts",
    //         "Jack of Hearts",
    //         "Queen of Hearts",
    //         "King of Hearts",
    //         "Ace of Spades",
    //         "Two of Spades",
    //         "Three of Spades",
    //         "Four of Spades",
    //         "Five of Spades",
    //         "Six of Spades",
    //         "Seven of Spades",
    //         "Eight of Spades",
    //         "Nine of Spades",
    //         "Ten of Spades",
    //         "Jack of Spades",
    //         "Queen of Spades",
    //         "King of Spades",
    //     ];
    //     expected_cards.sort();
    //     deck_as_strings.sort();

    //     assert!(expected_cards.iter().eq(deck_as_strings.iter()));
    // }

    // #[test]
    // fn test_hand_value() {
    //     let hand = Hand {
    //         cards: vec![
    //             Card {rank: Rank::Ten, suit: Suit::Hearts, inflated: true},
    //             Card {rank: Rank::Eight, suit: Suit::Hearts, inflated: true}
    //             ],
    //         bet: 10,
    //         state: crate::HandState::Playing
    //     };

    //     assert!(hand.value() == 18);
    // }
}
