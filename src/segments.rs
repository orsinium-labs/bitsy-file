use crate::*;
use alloc::string::ToString;
use alloc::{string::String, vec::Vec};
use core::str::FromStr;

pub enum Segment {
    Name(String),
    Version(Version),
    RoomFormat(RoomFormat),
    Font(Font, Option<String>),
    TextDirection(TextDirection),
    Palette(Palette),
    Room(Room, RoomType),
    Tile(Tile),
    Sprite(Sprite),
    Item(Item),
    Dialogue(Dialogue),
    Ending(Ending),
    Variable(Variable),
    FontData(String),
    Warning(Error),
}

pub struct Segments {
    parts: Vec<String>,
    emitted_name: bool,
}

impl Segments {
    #[must_use]
    pub fn new(string: &str) -> Self {
        let string = string.replace("\r\n", "\n");
        let string = string.trim_start_matches('\n').to_string();
        let parts = segments_from_str(&string);
        Self {
            parts,
            emitted_name: false,
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.parts.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    fn find_name(&mut self) -> Option<String> {
        let segment = self.parts.pop()?;

        // game names can be empty - so when we strip out the leading whitespace above,
        // it means that the first segment might not be the game name.
        // so, check if the first segment is actually the next segment of game data
        // to avoid setting the game name to "# BITSY VERSION 7.0" or something
        if segment.starts_with("\"\"\"") // multi-line game name
            ||
            (
                ! segment.starts_with("# BITSY VERSION ")
                &&
                ! segment.starts_with("! ROOM_FORMAT ")
                &&
                ! segment.starts_with("PAL ")
                &&
                ! segment.starts_with("DEFAULT_FONT ")
                &&
                ! segment.starts_with("TEXT_DIRECTION ")
            )
        {
            return Some(segment.to_string());
        }
        self.parts.push(segment);
        None
    }
}

impl Iterator for Segments {
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.emitted_name {
            self.emitted_name = true;
            if let Some(name) = self.find_name() {
                return Some(Segment::Name(name));
            }
        }
        while let Some(segment) = self.parts.pop() {
            if let Some(segment) = parse_segment(segment) {
                return Some(segment);
            }
        }
        None
    }
}

fn parse_segment(segment: String) -> Option<Segment> {
    if segment.starts_with("# BITSY VERSION") {
        let segment = segment.replace("# BITSY VERSION ", "");
        let result = Version::new(segment.as_str());
        let segment = if let Ok(v) = result {
            Segment::Version(v)
        } else {
            Segment::Warning(Error::Version)
        };
        return Some(segment);
    }
    if segment.starts_with("! ROOM_FORMAT") {
        let segment = segment.replace("! ROOM_FORMAT ", "");
        let room_format = RoomFormat::from(&segment).ok()?;
        return Some(Segment::RoomFormat(room_format));
    }
    let (first_word, _) = segment.split_once(' ')?;
    match first_word {
        "DEFAULT_FONT" => {
            let segment = segment.replace("DEFAULT_FONT ", "");
            let font = Font::from(&segment);
            let custom_data = if font == Font::Custom {
                Some(segment)
            } else {
                None
            };
            Some(Segment::Font(font, custom_data))
        }
        "TEXT_DIRECTION" => {
            if segment.trim() == "TEXT_DIRECTION RTL" {
                Some(Segment::TextDirection(TextDirection::RightToLeft))
            } else {
                None
            }
        }
        "PAL" => match Palette::from_str(&segment) {
            Ok((palette, _errors)) => Some(Segment::Palette(palette)),
            Err(err) => Some(Segment::Warning(err)),
        },
        "ROOM" | "SET" => {
            let room_type = if segment.starts_with("SET ") {
                RoomType::Set
            } else {
                RoomType::Room
            };
            let room = Room::from(segment.as_str());
            Some(Segment::Room(room, room_type))
        }
        "TIL" => {
            let tile = Tile::from(segment.as_str());
            Some(Segment::Tile(tile))
        }
        "SPR" => match Sprite::from_str(&segment) {
            Ok(sprite) => Some(Segment::Sprite(sprite)),
            Err(err) => Some(Segment::Warning(err)),
        },
        "ITM" => match Item::from_str(&segment) {
            Ok(item) => Some(Segment::Item(item)),
            Err(err) => Some(Segment::Warning(err)),
        },
        "DLG" => match Dialogue::from_str(&segment) {
            Ok(dialogue) => Some(Segment::Dialogue(dialogue)),
            Err(err) => Some(Segment::Warning(err)),
        },
        "END" => match Ending::from_str(&segment) {
            Ok(ending) => Some(Segment::Ending(ending)),
            Err(err) => Some(Segment::Warning(err)),
        },
        "VAR" => Some(Segment::Variable(Variable::from(segment.as_str()))),
        "FONT" => Some(Segment::FontData(segment)),
        _ => None,
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
    output.reverse();
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_segments() {
        let output = segments_from_str(include_str!("./test-resources/segments"));

        let expected = vec![
            "this is the last segment".to_string(),
            "DLG SEGMENT_3\n\"\"\"\nthis is a short \"long\" bit of text\n\"\"\"".to_string(),
            "this is a new segment\nthis is still the second segment\nblah\nblah".to_string(),
            "\"\"\"\nthe first segment is a long bit of text\n\n\nit contains empty lines\n\n\"\"\"".to_string(),
        ];

        assert_eq!(output, expected);
    }
}
