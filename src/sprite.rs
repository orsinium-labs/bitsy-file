use crate::image::animation_frames_from_str;
use crate::{AnimationFrames, Image, Position, optional_data_line};
use alloc::string::ToString;
use alloc::{format, string::String, vec::Vec};
use core::fmt;
use core::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sprite {
    pub id: String,
    pub name: Option<String>,
    pub animation_frames: Vec<Image>,
    pub dialogue_id: Option<String>,
    pub room_id: Option<String>,
    pub position: Option<Position>,
    pub colour_id: Option<u64>,
    pub items: Vec<String>,
}

impl Sprite {
    fn name_line(&self) -> String {
        optional_data_line("NAME", self.name.as_ref())
    }

    fn dialogue_line(&self) -> String {
        optional_data_line("DLG", self.dialogue_id.as_ref())
    }

    fn room_position_line(&self) -> String {
        if self.room_id.is_some() && self.position.is_some() {
            format!(
                "\nPOS {} {}",
                self.room_id.as_ref().unwrap(),
                self.position.as_ref().unwrap()
            )
        } else {
            "".to_string()
        }
    }

    fn colour_line(&self) -> String {
        optional_data_line("COL", self.colour_id.as_ref())
    }

    fn item_lines(&self) -> String {
        if self.items.is_empty() {
            "".to_string()
        } else {
            let lines: Vec<String> = self
                .items
                .iter()
                .map(|item| format!("ITM {}", item))
                .collect();

            format!("\n{}", lines.join("\n"))
        }
    }
}

impl FromStr for Sprite {
    type Err = crate::Error;

    fn from_str(str: &str) -> Result<Sprite, Self::Err> {
        let mut lines = str.lines();
        let Some(first_line) = lines.next() else {
            return Err(crate::Error::Sprite);
        };
        if !first_line.starts_with("SPR ") {
            return Err(crate::Error::Sprite);
        }

        let mut sprite = Sprite {
            id: first_line.replace("SPR ", ""),
            name: None,
            dialogue_id: None,
            room_id: None,
            position: None,
            colour_id: None,
            items: Vec::new(),
            animation_frames: Vec::new(),
        };

        let mut lines: Vec<_> = lines.collect();
        loop {
            let last_line = lines.pop().unwrap();
            let (first_word, _) = last_line.split_once(' ').unwrap_or_default();
            match first_word {
                "NAME" => {
                    sprite.name = Some(last_line.replace("NAME ", "").to_string());
                }
                "DLG" => {
                    sprite.dialogue_id = Some(last_line.replace("DLG ", "").to_string());
                }
                "POS" => {
                    let last_line = last_line.replace("POS ", "");
                    let room_position: Vec<&str> = last_line.split(' ').collect();
                    sprite.room_id = Some(room_position[0].to_string());

                    if room_position.len() < 2 {
                        return Err(crate::Error::Sprite);
                    }

                    if let Ok(pos) = Position::from_str(room_position[1]) {
                        sprite.position = Some(pos);
                    } else {
                        return Err(crate::Error::Sprite);
                    }
                }
                "COL" => {
                    sprite.colour_id = Some(last_line.replace("COL ", "").parse().unwrap());
                }
                "ITM" => {
                    sprite.items.push(last_line.replace("ITM ", ""));
                }
                _ => {
                    lines.push(last_line);
                    break;
                }
            }
        }

        sprite.items.reverse();
        sprite.animation_frames = animation_frames_from_str(&lines.join("\n"));
        Ok(sprite)
    }
}

impl fmt::Display for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SPR {}\n{}{}{}{}{}{}",
            self.id,
            self.animation_frames.to_string(),
            self.name_line(),
            self.dialogue_line(),
            self.room_position_line(),
            self.colour_line(),
            self.item_lines(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mock;

    #[test]
    fn sprite_from_string() {
        let output = Sprite::from_str(include_str!("test-resources/sprite")).unwrap();
        let expected = mock::sprite();

        assert_eq!(output, expected);
    }

    #[test]
    fn sprite_to_string() {
        assert_eq!(
            mock::sprite().to_string(),
            include_str!("test-resources/sprite")
        );
    }
}
