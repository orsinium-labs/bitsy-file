use crate::image::animation_frames_from_str;
use crate::{AnimationFrames, Image, optional_data_line};
use alloc::string::ToString;
use alloc::{string::String, vec::Vec};
use core::fmt;
use core::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Item {
    pub id: String,
    pub animation_frames: Vec<Image>,
    pub name: Option<String>,
    pub dialogue_id: Option<String>,
    pub colour_id: Option<u64>,
}

impl Item {
    fn name_line(&self) -> String {
        optional_data_line("NAME", self.name.as_ref())
    }

    fn dialogue_line(&self) -> String {
        optional_data_line("DLG", self.dialogue_id.as_ref())
    }

    fn colour_line(&self) -> String {
        optional_data_line("COL", self.colour_id.as_ref())
    }
}

impl FromStr for Item {
    type Err = crate::Error;

    fn from_str(str: &str) -> Result<Item, Self::Err> {
        let mut lines: Vec<&str> = str.lines().collect();

        if lines.is_empty() || !lines[0].starts_with("ITM ") {
            return Err(crate::Error::Item);
        }

        let id = lines[0].replace("ITM ", "");
        let mut name = None;
        let mut dialogue_id = None;
        let mut colour_id: Option<u64> = None;

        loop {
            let last_line = lines.pop().unwrap();

            if last_line.starts_with("NAME") {
                name = Some(last_line.replace("NAME ", "").to_string());
            } else if last_line.starts_with("DLG") {
                dialogue_id = Some(last_line.replace("DLG ", "").to_string());
            } else if last_line.starts_with("COL") {
                colour_id = Some(last_line.replace("COL ", "").parse().unwrap());
            } else {
                lines.push(last_line);
                break;
            }
        }

        let animation_frames = animation_frames_from_str(&lines[1..].join("\n"));

        Ok(Item {
            id,
            name,
            animation_frames,
            dialogue_id,
            colour_id,
        })
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ITM {}\n{}{}{}{}",
            self.id,
            self.animation_frames.to_string(),
            self.name_line(),
            self.dialogue_line(),
            self.colour_line(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mock;

    #[test]
    fn item_from_string() {
        let output = Item::from_str(include_str!("test-resources/item")).unwrap();
        let expected = mock::item();
        assert_eq!(output, expected);
    }

    #[test]
    fn item_to_string() {
        let output = mock::item().to_string();
        let expected = include_str!("test-resources/item").to_string();
        assert_eq!(output, expected);
    }
}
