use models::deck::Deck;

pub mod models;

pub fn new_deck() {
    let mut deck = Deck::new();
    let card = deck.draw();
    println!("{:#?}", card);
}
