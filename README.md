# bitsy-parser

![](https://img.shields.io/badge/license-MIT-blueviolet.svg)
[![Crates.io](https://img.shields.io/crates/v/bitsy-parser.svg)](https://crates.io/crates/bitsy-parser)

[Bitsy](http://ledoux.io/bitsy/editor.html) is a small game editor created by [Adam Le Doux](http://ledoux.io).
this crate provides a library and command-line utilities for working with Bitsy game data.
the version number follows Bitsy itself, so version 0.70.* targets Bitsy 7.0.

`bitsy-parser` is minimally invasive; unless you make any changes, 
an exported game should be identical to the imported game. however, this assumes your game data is valid.
most minor errors will simply be corrected on import (e.g. extraneous tiles in a room) 
but bigger problems may cause `bitsy-parser` to crash or fail. 

I have tested `bitsy-parser` on a dataset of over 1500 Bitsy games ranging from Bitsy 1.0 to Bitsy 7.1 
and have found that the vast majority of games can be imported without any problems.
so, I can almost guarantee that a Bitsy game will not be mangled by the parser, 
but I still recommend making backups of your game data periodically. 

## utilities

this crate provides some command-line tools for working with Bitsy game data. 

* `bitsy-dedupe`
* `bitsy-merge`
* `bitsy-validate`

the source for these can be found in `src/bin`.
if you have Cargo installed, you can install/update these utilities with `cargo install --force bitsy-parser`.
if your `.cargo/bin` directory is in your PATH, you will be able to use these utilities anywhere on your computer. 

## library

for use in your own Rust applications. can both parse and export Bitsy game data.

a simple example program:

```rust
use bitsy_parser::Game;
use std::{env, fs};

const SYNTAX_ERROR: &str = "No input path specified. Usage: `bitsy-validate filepath`";

fn main() {
    let input = env::args().nth(1).expect(SYNTAX_ERROR);
    Game::from(fs::read_to_string(input).unwrap()).unwrap();
    println!("OK!");
}
```

some more practical uses would be things like:

* convert images or other file formats to Bitsy assets 
* programmatically create Bitsy games
* a Bitsy game editor
