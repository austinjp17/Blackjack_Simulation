#![allow(dead_code)]
//! Provides a [`Deck`] implementation for simulating black jack games.

use core::slice::Iter;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom; // Required for shuffling the deck
use rand::{thread_rng, Rng, RngCore};
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
    pub fn value(&self, inflated: bool) -> u8 {
        match *self {
            Rank::Ace => match inflated {
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
struct Card {
    pub rank: Rank,
    pub suit: Suit,
    inflated: bool,
}

impl Card {
    pub fn is_blank(&self) -> bool {
        self.rank == Rank::Blank
    }

    pub fn is_inflated(&self) -> bool {
        self.inflated
    }

    pub fn deflate(&mut self) {
        self.inflated = false;
    }

    pub fn inflate(&mut self) {
        self.inflated = true;
    }

    pub fn value(&self) -> u8 {
        self.rank.value(self.inflated)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

// Define a deck
#[derive(Debug, Clone)]
pub struct Deck<T: Rng + Sized + Clone> {
    rng: T,
    cards: Vec<Card>,
}

impl<R> Deck<R>
where
    R: Rng + Sized + Clone,
{
    // Constructs a new, sorted standard deck
    pub fn standard(rng: R) -> Self {
        let mut cards = Vec::with_capacity(52);

        for &suit in Suit::iterator() {
            for &rank in Rank::iterator() {
                cards.push(Card {
                    rank,
                    suit,
                    inflated: true,
                });
            }
        }
        Self { cards, rng }
    }

    // Shuffles the deck in place
    pub fn shuffle_thread_rng(&mut self, rng: &mut R) {
        self.cards.shuffle(rng);
    }
}

pub struct MultiDeck<R: Rng + Sized + Clone> {
    decks: Vec<Deck<R>>,
    rng: R,
}

impl<R> MultiDeck<R>
where
    R: Rng + Sized + Clone,
{
    pub fn new(size: u8, rng: R) -> Self {
        let mut decks = Vec::with_capacity(size as usize);
        for _ in 0..size {
            decks.push(Deck::standard(rng.clone()));
        }
        Self { decks, rng }
    }

    pub fn shuffle_thread_rng(&mut self) {
        self.decks
            .iter_mut()
            .for_each(|d| d.shuffle_thread_rng(&mut self.rng));
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::thread;

    use rand::thread_rng;

    use crate::{Card, Deck, Rank, Suit};

    #[test]
    fn test_standard_deck_gen() {
        let deck = Deck::standard(thread_rng());
        assert_eq!(deck.cards.len(), 52);

        let mut expected_cards = Vec::new();
        for &suit in Suit::iterator() {
            for &rank in Rank::iterator() {
                expected_cards.push(Card {
                    rank,
                    suit,
                    inflated: true,
                });
            }
        }

        for (i, card) in deck.cards.iter().enumerate() {
            assert_eq!(card.to_string(), expected_cards[i].to_string());
        }
    }

    #[test]
    fn test_standard_deck_static() {
        let deck = Deck::standard(thread_rng());
        let mut deck_as_strings = deck
            .cards
            .iter()
            .map(|c| format!("{c}"))
            .collect::<Vec<String>>();
        assert_eq!(deck_as_strings.len(), 52);

        let mut expected_cards = vec![
            "Ace of Clubs",
            "Two of Clubs",
            "Three of Clubs",
            "Four of Clubs",
            "Five of Clubs",
            "Six of Clubs",
            "Seven of Clubs",
            "Eight of Clubs",
            "Nine of Clubs",
            "Ten of Clubs",
            "Jack of Clubs",
            "Queen of Clubs",
            "King of Clubs",
            "Ace of Diamonds",
            "Two of Diamonds",
            "Three of Diamonds",
            "Four of Diamonds",
            "Five of Diamonds",
            "Six of Diamonds",
            "Seven of Diamonds",
            "Eight of Diamonds",
            "Nine of Diamonds",
            "Ten of Diamonds",
            "Jack of Diamonds",
            "Queen of Diamonds",
            "King of Diamonds",
            "Ace of Hearts",
            "Two of Hearts",
            "Three of Hearts",
            "Four of Hearts",
            "Five of Hearts",
            "Six of Hearts",
            "Seven of Hearts",
            "Eight of Hearts",
            "Nine of Hearts",
            "Ten of Hearts",
            "Jack of Hearts",
            "Queen of Hearts",
            "King of Hearts",
            "Ace of Spades",
            "Two of Spades",
            "Three of Spades",
            "Four of Spades",
            "Five of Spades",
            "Six of Spades",
            "Seven of Spades",
            "Eight of Spades",
            "Nine of Spades",
            "Ten of Spades",
            "Jack of Spades",
            "Queen of Spades",
            "King of Spades",
        ];
        expected_cards.sort();
        deck_as_strings.sort();

        assert!(expected_cards.iter().eq(deck_as_strings.iter()));
    }
}
