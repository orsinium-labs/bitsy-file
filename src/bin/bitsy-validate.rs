use bitsy_parser::Game;
use std::{env, fs};

const SYNTAX_ERROR: &str = "No input path specified. Usage: `bitsy-validate filepath`";

fn main() {
    let input = env::args().nth(1).expect(SYNTAX_ERROR);
    Game::from(fs::read_to_string(input).unwrap()).unwrap();
    println!("OK!");
}
