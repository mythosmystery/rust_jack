use card_engine::models::{deck::Deck, player::Player};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

pub struct Game {
    pub player: Player,
    pub dealer: Player,
    pub deck: Deck,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::new("Player".to_string()),
            dealer: Player::new("Dealer".to_string()),
            deck: Deck::new(),
        }
    }
    pub fn game_loop(&mut self) {
        loop {
            self.init_turn();
            self.turn_loop();
            self.dealer.dealer_choice(&mut self.deck);
            println!("{}", self.get_result());
            println!("Dealer: {}", self.dealer.score());
            if Confirm::new()
                .with_prompt("Play again?")
                .interact()
                .unwrap()
            {
                self.deck = Deck::new();
                self.player.hand = Vec::new();
                self.dealer.hand = Vec::new();
            } else {
                break;
            }
        }
    }
    pub fn get_result(&self) -> String {
        if self.player.score() > 21 {
            "You busted!".to_string()
        } else if self.dealer.score() > 21 {
            "Dealer busted!".to_string()
        } else if self.player.score() > self.dealer.score() {
            "You win!".to_string()
        } else if self.player.score() < self.dealer.score() {
            "You lose!".to_string()
        } else {
            "Draw!".to_string()
        }
    }
    pub fn init_turn(&mut self) {
        self.player.draw(&mut self.deck);
        self.dealer.draw(&mut self.deck);
        self.player.draw(&mut self.deck);
        self.dealer.draw(&mut self.deck);
        println!("{:#?}", self.player.hand);
        println!("{}", self.player.score());
    }
    pub fn turn_loop(&mut self) {
        loop {
            if self.player.score() > 21 {
                break;
            }
            let items = vec!["Stay", "Hit"];
            let choice = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What would you like to do?")
                .default(0)
                .items(&items)
                .interact_opt()
                .unwrap()
                .unwrap_or(0);
            match choice {
                0 => {
                    println!("Stay");
                    break;
                }
                1 => {
                    println!("Hit");
                    self.player.draw(&mut self.deck);
                    println!("{:#?}", self.player.hand);
                    println!("Score: {}", self.player.score());
                }
                _ => {
                    println!("Invalid choice");
                    break;
                }
            }
        }
    }
}
