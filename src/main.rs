use deck::Deck;
use rand::thread_rng;
fn main() {
    let mut rng = thread_rng();
    let mut deck = Deck::standard();
    deck.shuffle_rng(&mut rng);

    println!("{:?}", deck);
}
