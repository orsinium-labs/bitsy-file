use bitsy_parser::Game;
use std::{env, fs};

const SYNTAX_ERROR: &str = "No main game specified. Usage: `bitsy-merge main.bitsy additional.bitsy output.bitsy`";

fn main() {
    let game_a = env::args().nth(1).expect(SYNTAX_ERROR);
    let game_b = env::args().nth(2).expect(SYNTAX_ERROR);
    let output = env::args().nth(3).expect(SYNTAX_ERROR);
    // todo allow numerous additional games

    let  (mut game_a, _) = Game::from(fs::read_to_string(game_a).unwrap()).unwrap();
    let  (    game_b, _) = Game::from(fs::read_to_string(game_b).unwrap()).unwrap();

    game_a.merge(&game_b);

    fs::write(output, game_a.to_string())
        .expect("Failed to write output file");
}
