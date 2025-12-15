use crate::image::animation_frames_from_str;
use crate::{optional_data_line, AnimationFrames, Image, Position};

use std::fmt;

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
                self.position.as_ref().unwrap().to_string()
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

    pub fn from_str(str: &str) -> Result<Sprite, crate::Error> {
        let mut lines: Vec<&str> = str.lines().collect();

        if lines.is_empty() || !lines[0].starts_with("SPR ") {
            return Err(crate::Error::Sprite);
        }

        let id = lines[0].replace("SPR ", "");
        let mut name = None;
        let mut dialogue_id: Option<String> = None;
        let mut room_id: Option<String> = None;
        let mut position: Option<Position> = None;
        let mut colour_id: Option<u64> = None;
        let mut items: Vec<String> = Vec::new();

        loop {
            let last_line = lines.pop().unwrap();

            if last_line.starts_with("NAME") {
                name = Some(last_line.replace("NAME ", "").to_string());
            } else if last_line.starts_with("DLG") {
                dialogue_id = Some(last_line.replace("DLG ", "").to_string());
            } else if last_line.starts_with("POS") {
                let last_line = last_line.replace("POS ", "");
                let room_position: Vec<&str> = last_line.split(' ').collect();
                room_id = Some(room_position[0].to_string());

                if room_position.len() < 2 {
                    return Err(crate::Error::Sprite);
                }

                if let Ok(pos) = Position::from_str(room_position[1]) {
                    position = Some(pos);
                } else {
                    return Err(crate::Error::Sprite);
                }
            } else if last_line.starts_with("COL") {
                colour_id = Some(last_line.replace("COL ", "").parse().unwrap());
            } else if last_line.starts_with("ITM") {
                items.push(last_line.replace("ITM ", ""));
            } else {
                lines.push(last_line);
                break;
            }
        }

        items.reverse();

        let animation_frames = animation_frames_from_str(&lines[1..].join("\n"));

        Ok(Sprite {
            id,
            name,
            animation_frames,
            dialogue_id,
            room_id,
            position,
            colour_id,
            items,
        })
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
    use crate::{mock, Sprite};

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
