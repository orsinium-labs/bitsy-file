use bitsy_parser::Game;
use std::{env, fs};

const SYNTAX_ERROR: &str = "No game data specified. Usage: `bitsy-dedupe input.bitsy output.bitsy`";

fn main() {
    let game   = env::args().nth(1).expect(SYNTAX_ERROR);
    let output = env::args().nth(2).expect(SYNTAX_ERROR);

    let (mut game, _err) = Game::from(fs::read_to_string(game).unwrap()).unwrap();

    game.dedupe_tiles();

    fs::write(output, game.to_string()).expect("Failed to write output file");
}
