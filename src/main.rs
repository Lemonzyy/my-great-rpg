mod util;
mod game;
mod team;
mod character;
mod weapon;

use game::Game;

fn main() {
    let mut g = Game::new();
    g.init();
    g.start();

    println!("finished.")
}