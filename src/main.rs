mod error;
mod game;
mod generator;
mod problem;
mod utils;

use anyhow::Result;
use game::{Game, SimpleGame};
use generator::BasicGenerator;

use utils::UNICODE_DOT;

fn main() {
    env_logger::init();

    println!("Let's do some math!");

    loop {
        let mut game = SimpleGame::new(Box::new(BasicGenerator::new()));

        game.prepare().expect("Error preparing game.");

        game.start().expect("Error preparing game.");

        game.play().expect("Error playing game.");

        game.end().expect("Error ending game.");

        println!("\n{UNICODE_DOT} Play again? (Y/N)");

        let should_continue = utils::read_input::<String>()
            .expect("Error reading input.")
            .trim()
            .to_lowercase();

        if should_continue != "y" {
            break;
        }
    }
}
