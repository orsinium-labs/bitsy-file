use bitsy_parser::Game;
use std::{env, fs};

const SYNTAX_ERROR: &str = "Usage: `bitsy-parse input.bitsy output.bitsy`";

/// simply parses and re-exports a game. use to test whether output matches input.
fn main() {
    let input  = env::args().nth(1).expect(SYNTAX_ERROR);
    let output = env::args().nth(2).expect(SYNTAX_ERROR);

    let (game, _err) = Game::from(fs::read_to_string(input).unwrap()).unwrap();

    fs::write(output, game.to_string()).expect("Failed to write output file");
}
