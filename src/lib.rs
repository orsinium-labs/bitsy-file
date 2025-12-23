#![cfg_attr(not(test), no_std)]
#![allow(clippy::to_string_trait_impl)]
extern crate alloc;

use alloc::string::ToString;
use alloc::{format, string::String, vec, vec::Vec};
use core::fmt::Display;

mod colour;
mod dialogue;
mod ending;
mod error;
mod exit;
mod frames;
mod game;
mod image;
mod instance;
mod item;
pub mod mock;
mod palette;
mod position;
mod room;
mod sprite;
mod test_omnibus;
mod text;
mod tile;
mod variable;

pub use colour::*;
pub use dialogue::*;
pub use ending::*;
pub use error::*;
pub use exit::*;
pub use frames::*;
pub use game::*;
pub use image::*;
pub use instance::*;
pub use item::*;
pub use palette::*;
pub use position::*;
pub use room::*;
pub use sprite::*;
pub use text::*;
pub use tile::*;
pub use variable::*;

fn to_base36(mut x: u32) -> String {
    let mut result = Vec::new();
    loop {
        let m = x % 36;
        x /= 36;
        result.push(core::char::from_digit(m, 36).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

/// e.g. `\nNAME DLG_0`
fn optional_data_line<T: Display>(label: &str, item: Option<T>) -> String {
    if item.is_some() {
        format!("\n{} {}", label, item.unwrap())
    } else {
        "".to_string()
    }
}

fn segments_from_str(str: &str) -> Vec<String> {
    // this is pretty weird but a dialogue can just have an empty line followed by a name
    // however, on entering two empty lines, dialogue will be wrapped in triple quotation marks
    // so, handle this here
    let string = str.replace("\n\nNAME", "\n\"\"\"\n\"\"\"\nNAME");

    let mut output: Vec<String> = Vec::new();
    // are we inside `"""\n...\n"""`? if so, ignore empty lines
    let mut inside_escaped_block = false;
    let mut current_segment: Vec<String> = Vec::new();

    for line in string.lines() {
        if line == "\"\"\"" {
            inside_escaped_block = !inside_escaped_block;
        }

        if line.is_empty() && !inside_escaped_block {
            output.push(current_segment.join("\n"));
            current_segment = Vec::new();
        } else {
            current_segment.push(line.to_string());
        }
    }

    output.push(current_segment.join("\n"));

    output
}
