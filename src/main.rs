use game::Game;

mod util;
mod game;
mod team;
mod character;
mod weapon;

fn main() {
    Game::new()
        .init()
        .start();

    println!("finished.")
}