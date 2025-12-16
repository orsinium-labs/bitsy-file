#![cfg_attr(not(test), no_std)]
#![allow(clippy::to_string_trait_impl)]
extern crate alloc;

use alloc::string::ToString;
use alloc::{format, string::String, vec, vec::Vec};
use core::fmt::Display;

pub mod colour;
pub mod dialogue;
pub mod ending;
pub mod error;
pub mod exit;
pub mod game;
pub mod image;
pub mod item;
pub mod mock;
pub mod palette;
pub mod position;
pub mod room;
pub mod sprite;
pub mod test_omnibus;
pub mod text;
pub mod tile;
pub mod variable;

pub use colour::Colour;
pub use dialogue::Dialogue;
pub use ending::Ending;
pub use error::Error;
pub use exit::*;
pub use game::*;
pub use image::Image;
pub use item::Item;
pub use palette::Palette;
pub use position::Position;
pub use room::Room;
pub use sprite::Sprite;
pub use text::*;
pub use tile::Tile;
pub use variable::Variable;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instance {
    pub position: Position,
    pub id: String, // item / ending id
}

/// a Room can have many Exits in different positions,
/// optionally with a transition and dialogue
/// todo make a from_str() function for this
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExitInstance {
    pub position: Position,
    pub exit: Exit,
    pub transition: Option<Transition>,
    pub dialogue_id: Option<String>,
}

pub trait AnimationFrames {
    fn to_string(&self) -> String;
}

impl AnimationFrames for Vec<Image> {
    fn to_string(&self) -> String {
        let mut string = String::new();
        let last_frame = self.len() - 1;

        for (i, frame) in self.iter().enumerate() {
            string.push_str(&frame.to_string());

            if i < last_frame {
                string.push_str("\n>\n");
            }
        }

        string
    }
}

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

/// tries to use an existing ID - if it is already in use, generate a new one
/// then return the ID (either original or new)
/// todo refactor (unnecessary clones etc.)
fn try_id(ids: &[String], id: &str) -> String {
    if is_id_available(ids, id) {
        id.to_string()
    } else {
        new_unique_id(ids)
    }
}

fn is_id_available(ids: &[String], id: &str) -> bool {
    !ids.iter().any(|v| v == id)
}

/// e.g. pass all tile IDs into this to get a new non-conflicting tile ID
fn new_unique_id(ids: &[String]) -> String {
    let mut new_id: u32 = 0;
    while ids.contains(&to_base36(new_id)) {
        new_id += 1;
    }
    to_base36(new_id)
}

pub trait Quote {
    fn quote(&self) -> String;
}

impl Quote for String {
    fn quote(&self) -> String {
        format!("\"\"\"\n{}\n\"\"\"", self)
    }
}

pub trait Unquote {
    fn unquote(&self) -> String;
}

impl Unquote for String {
    fn unquote(&self) -> String {
        self.trim_matches('\"').trim_matches('\n').to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_base36() {
        assert_eq!(to_base36(37), "11");
    }

    #[test]
    fn test_optional_data_line() {
        let output = optional_data_line("NAME", mock::item().name);
        assert_eq!(output, "\nNAME door");
    }

    #[test]
    fn string_to_segments() {
        let output = segments_from_str(include_str!("./test-resources/segments"));

        let expected = vec![
            "\"\"\"\nthe first segment is a long bit of text\n\n\nit contains empty lines\n\n\"\"\"".to_string(),
            "this is a new segment\nthis is still the second segment\nblah\nblah".to_string(),
            "DLG SEGMENT_3\n\"\"\"\nthis is a short \"long\" bit of text\n\"\"\"".to_string(),
            "this is the last segment".to_string(),
        ];

        assert_eq!(output, expected);
    }

    #[test]
    fn quote() {
        let output = "this is a string.\nIt has 2 lines".to_string().quote();
        let expected = "\"\"\"\nthis is a string.\nIt has 2 lines\n\"\"\"";
        assert_eq!(output, expected);
    }

    #[test]
    fn unquote() {
        let output = "\"\"\"\nwho the fuck is scraeming \"LOG OFF\" at my house.\nshow yourself, coward.\ni will never log off\n\"\"\"".to_string().unquote();
        let expected = "who the fuck is scraeming \"LOG OFF\" at my house.\nshow yourself, coward.\ni will never log off";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_try_id() {
        // does a conflict generate a new ID?
        assert_eq!(try_id(&["0".to_string(), "1".to_string()], "1"), "2");
        // with no conflict, does the ID remain the same?
        assert_eq!(try_id(&["0".to_string(), "1".to_string()], "3"), "3");
    }

    #[test]
    fn test_new_unique_id() {
        // start
        assert_eq!(
            new_unique_id(&["1".to_string(), "z".to_string()]),
            "0".to_string()
        );
        // middle
        assert_eq!(
            new_unique_id(&["0".to_string(), "2".to_string()]),
            "1".to_string()
        );
        // end
        assert_eq!(
            new_unique_id(&["0".to_string(), "1".to_string()]),
            "2".to_string()
        );
        // check sorting
        assert_eq!(
            new_unique_id(&["1".to_string(), "0".to_string()]),
            "2".to_string()
        );
        // check deduplication
        assert_eq!(
            new_unique_id(&["0".to_string(), "0".to_string(), "1".to_string()]),
            "2".to_string()
        );
    }
}
