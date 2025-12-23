#![cfg_attr(not(test), no_std)]
#![allow(clippy::to_string_trait_impl)]
extern crate alloc;

use alloc::string::ToString;
use alloc::{format, string::String, vec::Vec};
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
mod segments;
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
pub use segments::*;
pub use sprite::*;
pub use text::*;
pub use tile::*;
pub use variable::*;

/// e.g. `\nNAME DLG_0`
fn optional_data_line<T: Display>(label: &str, item: Option<T>) -> String {
    if let Some(item) = item {
        format!("\n{} {}", label, item)
    } else {
        "".to_string()
    }
}
