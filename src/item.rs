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
        let mut lines = str.lines();
        let Some(first_line) = lines.next() else {
            return Err(crate::Error::Sprite);
        };
        if !first_line.starts_with("ITM ") {
            return Err(crate::Error::Sprite);
        }

        let mut item = Item {
            id: first_line.replace("ITM ", ""),
            name: None,
            dialogue_id: None,
            colour_id: None,
            animation_frames: Vec::new(),
        };

        {
            let image = Image::from_lines(&mut lines)?;
            item.animation_frames.push(image);
        }

        loop {
            let Some(line) = lines.next() else {
                break;
            };
            let (first_word, rest) = line.split_once(' ').unwrap_or((line, ""));
            match first_word {
                "NAME" => {
                    item.name = Some(rest.to_string());
                }
                "DLG" => {
                    item.dialogue_id = Some(rest.to_string());
                }
                "COL" => {
                    item.colour_id = Some(rest.parse().unwrap());
                }
                ">" => {
                    let image = Image::from_lines(&mut lines)?;
                    item.animation_frames.push(image);
                }
                _ => {}
            }
        }

        Ok(item)
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
