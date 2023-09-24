//! Provides a [`Deck`] implementation for simulating black jack games.

use core::slice::Iter;
use rand::seq::SliceRandom; // Required for shuffling the deck
use rand::thread_rng;

/// Card Suit representation
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Suit {
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

/// Card rank representation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Rank {
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

// Define a card as a combination of rank and suit
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Card {
    rank: Rank,
    suit: Suit,
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

// Define a deck
#[derive(Debug, Clone)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    // Constructs a new, sorted standard deck
    pub fn standard() -> Self {
        let mut cards = Vec::with_capacity(52);
        let suits = Suit::iterator();
        let ranks = Rank::iterator();

        for &suit in Suit::iterator() {
            for &rank in Rank::iterator() {
                cards.push(Card {
                    rank,
                    suit,
                    inflated: true,
                });
            }
        }
        Self { cards }
    }

    // Shuffles the deck in place
    pub fn shuffle_thread_rng(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    // Constructs a blackjack deck with `num_decks` standard decks shuffled together
    fn blackjack(num_decks: usize) -> Self {
        let standard_deck = Self::standard();
        let mut cards = Vec::with_capacity(52 * num_decks);
        for _ in 0..num_decks {
            cards.extend(standard_deck.cards.iter());
        }
        let mut blackjack_deck = Self { cards };
        blackjack_deck.shuffle_thread_rng();
        blackjack_deck
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn valid_simple_deck() {}
}
