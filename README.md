# bitsy-file

Rust crate for parsing [bitsy](https://www.bitsy.org/) scripts.

Base on [bitsy-parser](https://crates.io/crates/bitsy-parser) by [Max Bradbury](https://crates.io/users/synth-ruiner).

See also [bitsy-script](https://github.com/orsinium-labs/bitsy-script) for parsing and interpreting bitsy dialogs and [firefly-bitsy](https://github.com/firefly-zero/firefly-bitsy) for running bitsy games on [Firefly Zero](https://fireflyzero.com/).

## Installation

```bash
cargo add bitsy-file
```

## Usage

```rust
let content: &str = todo!();
let game = bitsy_file::Game::from(content)?;
for warning in &game.warnings {
    todo!();
}
```
