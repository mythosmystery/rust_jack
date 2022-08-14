use super::card::{Card, Rank, Suit};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, PartialEq)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(suit, rank));
            }
        }
        Deck { cards }
    }
    pub fn draw(&mut self) -> Card {
        if self.cards.len() > 0 {
            let num = rand::random::<u8>() % self.cards.len() as u8;
            self.cards.remove(num as usize)
        } else {
            Card::rand()
        }
    }
}
