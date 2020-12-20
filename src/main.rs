use crate::{
    game::Game,
    util::read_line
};

mod util;
mod game;
mod team;
mod character;
mod weapon;

fn main() {
    Game::new()
        .init()
        .start();

    println!("Press a key to exit!");
    read_line();
}