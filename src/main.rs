use deck::Deck;
fn main() {
    let mut deck = Deck::standard();
    deck.shuffle_thread_rng();

    println!("{:?}", deck);
}
