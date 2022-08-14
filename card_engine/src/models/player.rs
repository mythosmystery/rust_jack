use super::{card::Card, deck::Deck};

pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub wins: u8,
    pub losses: u8,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            hand: Vec::new(),
            wins: 0,
            losses: 0,
        }
    }
    pub fn draw(&mut self, deck: &mut Deck) {
        self.hand.push(deck.draw());
    }
    pub fn score(&self) -> u8 {
        self.hand.iter().fold(0, |acc, card| acc + card.score())
    }
    pub fn dealer_choice(&mut self, deck: &mut Deck) {
        while self.score() < 17 {
            self.draw(deck);
        }
    }
}
