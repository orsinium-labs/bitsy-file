use crate::{AnimationFrames, Image, optional_data_line};
use alloc::string::ToString;
use alloc::{format, string::String, vec::Vec};
use core::str::FromStr;

#[derive(Clone, Debug, Eq)]
pub struct Tile {
    pub id: String,
    pub name: Option<String>,
    /// this is "optional" in that a tile can have `WAL true`, `WAL false` or neither
    /// obviously Some(false) is the same as None but we want to preserve the original formatting
    pub wall: Option<bool>,
    pub animation_frames: Vec<Image>,
    pub colour_id: Option<u64>,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.wall == other.wall
            && self.animation_frames == other.animation_frames
            && self.colour_id == other.colour_id
    }
}

impl Tile {
    fn name_line(&self) -> String {
        optional_data_line("NAME", self.name.as_ref())
    }

    fn wall_line(&self) -> String {
        if self.wall.is_some() {
            format!("\nWAL {}", self.wall.unwrap())
        } else {
            "".to_string()
        }
    }

    fn colour_line(&self) -> String {
        if self.colour_id.is_some() {
            format!("\nCOL {}", self.colour_id.unwrap())
        } else {
            "".to_string()
        }
    }

    // todo refactor

    pub fn invert(&mut self) {
        self.animation_frames = self
            .animation_frames
            .iter()
            .map(|frame: &Image| {
                let mut image = frame.clone();
                image.invert();
                image
            })
            .collect()
    }

    pub fn flip(&mut self) {
        self.animation_frames = self
            .animation_frames
            .iter()
            .map(|frame: &Image| {
                let mut image = frame.clone();
                image.flip();
                image
            })
            .collect()
    }

    pub fn mirror(&mut self) {
        self.animation_frames = self
            .animation_frames
            .iter()
            .map(|frame: &Image| {
                let mut image = frame.clone();
                image.mirror();
                image
            })
            .collect()
    }

    pub fn rotate(&mut self) {
        self.animation_frames = self
            .animation_frames
            .iter()
            .map(|frame: &Image| {
                let mut image = frame.clone();
                image.rotate();
                image
            })
            .collect()
    }
}

impl FromStr for Tile {
    type Err = crate::Error;

    fn from_str(str: &str) -> Result<Tile, crate::Error> {
        let mut lines = str.lines();
        let Some(first_line) = lines.next() else {
            return Err(crate::Error::Tile);
        };
        let Some(id) = first_line.strip_prefix("TIL ") else {
            return Err(crate::Error::Sprite);
        };
        let mut tile = Tile {
            id: id.to_string(),
            wall: None,
            name: None,
            colour_id: None,
            animation_frames: Vec::new(),
        };

        {
            let image = Image::from_lines(&mut lines)?;
            tile.animation_frames.push(image);
        }

        loop {
            let Some(line) = lines.next() else {
                break;
            };
            let (first_word, rest) = line.split_once(' ').unwrap_or((line, ""));
            match first_word {
                "WAL" => {
                    tile.wall = Some(rest == "true");
                }
                "NAME" => {
                    tile.name = Some(rest.to_string());
                }
                "COL" => {
                    tile.colour_id = Some(rest.parse().unwrap());
                }
                ">" => {
                    let image = Image::from_lines(&mut lines)?;
                    tile.animation_frames.push(image);
                }
                _ => {}
            }
        }
        Ok(tile)
    }
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        format!(
            "TIL {}\n{}{}{}{}",
            self.id,
            self.animation_frames.to_string(),
            self.name_line(),
            self.wall_line(),
            self.colour_line(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mock;
    use alloc::vec;

    #[test]
    fn tile_from_string() {
        let output = Tile::from_str(include_str!("test-resources/tile")).unwrap();
        let expected = Tile {
            id: "z".to_string(),
            name: Some("concrete 1".to_string()),
            wall: Some(true),
            animation_frames: vec![Image {
                pixels: vec![1; 64],
            }],
            colour_id: None,
        };

        assert_eq!(output, expected);
    }

    #[test]
    fn tile_to_string() {
        let output = Tile {
            id: "7a".to_string(),
            name: Some("chequers".to_string()),
            wall: None,
            animation_frames: vec![mock::image::chequers_1(), mock::image::chequers_2()],
            colour_id: None,
        }
        .to_string();

        let expected = include_str!("test-resources/tile-chequers").to_string();

        assert_eq!(output, expected);
    }

    #[test]
    fn partial_eq() {
        let tile_a = crate::mock::tile_default();
        let mut tile_b = crate::mock::tile_default();
        tile_b.id = "0".to_string();
        assert_eq!(tile_a, tile_b);
        tile_b.name = None;
        assert_eq!(tile_a, tile_b);
    }
}
