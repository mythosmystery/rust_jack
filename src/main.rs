use game::Game;

mod game;

fn main() {
    let mut game = Game::new();
    game.game_loop();
}
