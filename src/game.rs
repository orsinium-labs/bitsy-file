use crate::*;
use alloc::borrow::ToOwned;
use alloc::string::ToString;
use alloc::{format, string::String, vec::Vec};
use core::fmt;

/// in very early versions of Bitsy, room tiles were defined as single alphanumeric characters -
/// so there was a maximum of 36 unique tiles. later versions are comma-separated.
/// RoomFormat is implemented here so we can save in the original format.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum RoomFormat {
    Contiguous,
    CommaSeparated,
}

#[derive(Debug)]
pub struct InvalidRoomFormat;

impl RoomFormat {
    pub fn from(str: &str) -> Result<RoomFormat, InvalidRoomFormat> {
        match str {
            "0" => Ok(RoomFormat::Contiguous),
            "1" => Ok(RoomFormat::CommaSeparated),
            _ => Err(InvalidRoomFormat),
        }
    }
}

impl fmt::Display for RoomFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                RoomFormat::Contiguous => 0,
                RoomFormat::CommaSeparated => 1,
            }
        )
    }
}

/// in very early versions of Bitsy, a room was called a "set"
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub enum RoomType {
    #[default]
    Room,
    Set,
}

impl ToString for RoomType {
    fn to_string(&self) -> String {
        match &self {
            RoomType::Set => "SET",
            RoomType::Room => "ROOM",
        }
        .to_string()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

#[derive(Debug)]
pub enum VersionError {
    MissingParts,
    ExtraneousParts,
    MalformedInteger,
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VersionError::MissingParts => "Not enough parts supplied for version",
                VersionError::ExtraneousParts => "Too many parts supplied for version",
                VersionError::MalformedInteger => "Version did not contain valid integers",
            }
        )
    }
}

impl core::error::Error for VersionError {}

impl Version {
    pub fn new(str: &str) -> Result<Version, VersionError> {
        let parts: Vec<&str> = str.split('.').collect();

        if parts.len() < 2 {
            Err(VersionError::MissingParts)
        } else if parts.len() > 2 {
            Err(VersionError::ExtraneousParts)
        } else if let (Ok(major), Ok(minor)) = (parts[0].parse(), parts[1].parse()) {
            Ok(Version { major, minor })
        } else {
            Err(VersionError::MalformedInteger)
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Game {
    pub name: String,
    pub version: Option<Version>,
    pub room_format: Option<RoomFormat>,
    pub(crate) room_type: RoomType,
    pub font: Font,
    pub custom_font: Option<String>, // used if font is Font::Custom
    pub text_direction: TextDirection,
    pub palettes: Vec<Palette>,
    pub rooms: Vec<Room>,
    pub tiles: Vec<Tile>,
    pub sprites: Vec<Sprite>,
    pub items: Vec<Item>,
    pub dialogues: Vec<Dialogue>,
    pub endings: Vec<Ending>,
    pub variables: Vec<Variable>,
    pub font_data: Option<String>, // todo make this an actual struct for parsing
    pub warnings: Vec<crate::Error>,
}

impl Game {
    pub fn from(string: &str) -> Result<Game, crate::error::NotFound> {
        if string.trim() == "" {
            return Err(crate::error::NotFound::Anything);
        }

        let mut game = Game::default();
        let segments = Segments::new(string);
        for segment in segments {
            game.push_segment(segment)
        }
        // if !avatar_exists {
        //     warnings.push(crate::Error::Game {
        //         missing: NotFound::Avatar,
        //     });
        // }
        Ok(game)
    }

    pub fn push_segment(&mut self, segment: Segment) {
        match segment {
            Segment::Name(name) => self.name = name,
            Segment::Version(version) => self.version = Some(version),
            Segment::RoomFormat(room_format) => self.room_format = Some(room_format),
            Segment::Font(font, data) => {
                self.font = font;
                if let Some(data) = data {
                    self.custom_font = Some(data);
                }
            }
            Segment::TextDirection(text_direction) => self.text_direction = text_direction,
            Segment::Palette(palette) => self.palettes.push(palette),
            Segment::Room(room, room_type) => {
                if room_type == RoomType::Set {
                    self.room_type = room_type;
                }
                self.rooms.push(room)
            }
            Segment::Tile(tile) => self.tiles.push(tile),
            Segment::Sprite(sprite) => self.sprites.push(sprite),
            Segment::Item(item) => self.items.push(item),
            Segment::Dialogue(dialogue) => self.dialogues.push(dialogue),
            Segment::Ending(ending) => self.endings.push(ending),
            Segment::Variable(variable) => self.variables.push(variable),
            Segment::FontData(data) => self.font_data = Some(data),
            Segment::Warning(error) => self.warnings.push(error),
        };
    }

    pub fn get_sprite(&self, id: &str) -> Option<&Sprite> {
        self.sprites.iter().find(|sprite| sprite.id == id)
    }

    pub fn get_tile(&self, id: &str) -> Option<&Tile> {
        self.tiles.iter().find(|tile| tile.id == id)
    }

    pub fn get_item(&self, id: &str) -> Option<&Item> {
        self.items.iter().find(|item| item.id == id)
    }

    pub fn get_room(&self, id: &str) -> Option<&Room> {
        self.rooms.iter().find(|room| room.id == id)
    }

    pub fn get_avatar(&self) -> Option<&Sprite> {
        self.get_sprite("A")
    }

    pub fn get_room_tiles(&self, room_id: &str) -> Vec<&Tile> {
        let Some(room) = self.get_room(room_id) else {
            return Vec::new();
        };
        let mut tile_ids = room.tiles.clone();
        tile_ids.sort();
        tile_ids.dedup();
        // remove 0 as this isn't a real tile
        let zero_index = tile_ids.iter().position(|i| i == "0");
        if let Some(zero_index) = zero_index {
            tile_ids.remove(zero_index);
        }

        let mut tiles: Vec<&Tile> = Vec::new();
        for id in tile_ids {
            if let Some(tile) = self.get_tile(&id) {
                tiles.push(tile);
            }
        }
        tiles
    }
}

impl ToString for Game {
    fn to_string(&self) -> String {
        let mut segments: Vec<String> = Vec::new();

        // todo refactor

        for palette in &self.palettes {
            segments.push(palette.to_string());
        }

        for room in &self.rooms {
            segments.push(room.to_string(self.room_format(), self.room_type));
        }

        for tile in &self.tiles {
            segments.push(tile.to_string());
        }

        for sprite in &self.sprites {
            segments.push(sprite.to_string());
        }

        for item in &self.items {
            segments.push(item.to_string());
        }

        for dialogue in &self.dialogues {
            // this replacement is silly but see segments_from_string() for explanation
            segments.push(dialogue.to_string().replace("\"\"\"\n\"\"\"", ""));
        }

        for ending in &self.endings {
            segments.push(ending.to_string());
        }

        for variable in &self.variables {
            segments.push(variable.to_string());
        }

        if self.font_data.is_some() {
            segments.push(self.font_data.to_owned().unwrap())
        }

        format!(
            "{}{}{}{}{}\n\n{}\n\n",
            &self.name,
            &self.version_line(),
            &self.room_format_line(),
            &self.font_line(),
            &self.text_direction_line(),
            segments.join("\n\n"),
        )
    }
}

impl Game {
    // todo dedupe
    pub fn palette_ids(&self) -> Vec<String> {
        self.palettes
            .iter()
            .map(|palette| palette.id.clone())
            .collect()
    }

    pub fn tile_ids(&self) -> Vec<String> {
        self.tiles.iter().map(|tile| tile.id.clone()).collect()
    }

    pub fn sprite_ids(&self) -> Vec<String> {
        self.sprites
            .iter()
            .map(|sprite| sprite.id.clone())
            .collect()
    }
    pub fn room_ids(&self) -> Vec<String> {
        self.rooms.iter().map(|room| room.id.clone()).collect()
    }

    pub fn item_ids(&self) -> Vec<String> {
        self.items.iter().map(|item| item.id.clone()).collect()
    }

    pub fn dialogue_ids(&self) -> Vec<String> {
        self.dialogues
            .iter()
            .map(|dialogue| dialogue.id.clone())
            .collect()
    }

    pub fn ending_ids(&self) -> Vec<String> {
        self.endings
            .iter()
            .map(|ending| ending.id.clone())
            .collect()
    }

    pub fn variable_ids(&self) -> Vec<String> {
        self.variables
            .iter()
            .map(|variable| variable.id.clone())
            .collect()
    }

    // todo dedupe?

    pub fn new_palette_id(&self) -> String {
        new_unique_id(&self.palette_ids())
    }

    /// first available tile ID.
    /// e.g. if current tile IDs are [0, 2, 3] the result will be `1`
    ///      if current tile IDs are [0, 1, 2] the result will be `3`
    pub fn new_tile_id(&self) -> String {
        let mut ids = self.tile_ids();
        // don't allow 0 - this is a reserved ID for an implicit background tile
        ids.push("0".to_string());
        new_unique_id(&ids)
    }

    pub fn new_sprite_id(&self) -> String {
        new_unique_id(&self.sprite_ids())
    }

    pub fn new_room_id(&self) -> String {
        new_unique_id(&self.room_ids())
    }

    pub fn new_item_id(&self) -> String {
        new_unique_id(&self.item_ids())
    }

    pub fn new_dialogue_id(&self) -> String {
        new_unique_id(&self.dialogue_ids())
    }

    pub fn new_ending_id(&self) -> String {
        new_unique_id(&self.ending_ids())
    }

    pub fn new_variable_id(&self) -> String {
        new_unique_id(&self.variable_ids())
    }

    pub fn get_palette(&self, id: &str) -> Option<&Palette> {
        self.palettes.iter().find(|palette| palette.id == id)
    }

    /// todo refactor?
    pub fn get_tile_id(&self, matching_tile: &Tile) -> Option<String> {
        for tile in &self.tiles {
            if tile == matching_tile {
                return Some(tile.id.clone());
            }
        }

        None
    }

    pub fn find_tile_with_animation(&self, animation: &[Image]) -> Option<&Tile> {
        self.tiles
            .iter()
            .find(|&tile| tile.animation_frames.as_slice() == animation)
    }

    /// adds a palette safely and returns the ID
    pub fn add_palette(&mut self, mut palette: Palette) -> String {
        let new_id = try_id(&self.palette_ids(), &palette.id);
        if new_id != palette.id {
            palette.id = new_id.clone();
        }
        self.palettes.push(palette);
        new_id
    }

    /// adds a tile safely and returns the ID
    pub fn add_tile(&mut self, mut tile: Tile) -> String {
        if tile.id == "0" || self.tile_ids().contains(&tile.id) {
            let new_id = self.new_tile_id();
            if new_id != tile.id {
                tile.id = new_id;
            }
        }

        let id = tile.id.clone();
        self.tiles.push(tile);
        id
    }

    /// adds a sprite safely and returns the ID
    pub fn add_sprite(&mut self, mut sprite: Sprite) -> String {
        let new_id = try_id(&self.sprite_ids(), &sprite.id);
        if new_id != sprite.id {
            sprite.id = new_id.clone();
        }
        self.sprites.push(sprite);
        new_id
    }

    /// adds an item safely and returns the ID
    pub fn add_item(&mut self, mut item: Item) -> String {
        let new_id = try_id(&self.item_ids(), &item.id);
        if new_id != item.id {
            item.id = new_id.clone();
        }
        self.items.push(item);
        new_id
    }

    /// adds a dialogue safely and returns the ID
    pub fn add_dialogue(&mut self, mut dialogue: Dialogue) -> String {
        let new_id = try_id(&self.dialogue_ids(), &dialogue.id);
        if new_id != dialogue.id {
            dialogue.id = new_id.clone();
        }
        self.dialogues.push(dialogue);
        new_id
    }

    /// adds an ending safely and returns the ID
    pub fn add_ending(&mut self, mut ending: Ending) -> String {
        let new_id = try_id(&self.ending_ids(), &ending.id);
        if new_id != ending.id {
            ending.id = new_id.clone();
        }
        self.endings.push(ending);
        new_id
    }

    /// Safely adds a room and returns the room ID (a new ID will be generated if clashing)
    /// You will need to be mindful that the room's palette, tile, exit and ending IDs
    /// will be valid after adding.
    pub fn add_room(&mut self, mut room: Room) -> String {
        let new_id = try_id(&self.room_ids(), &room.id);
        if new_id != room.id {
            room.id = new_id.clone();
        }
        self.rooms.push(room);
        new_id
    }

    pub fn add_variable(&mut self, mut variable: Variable) -> String {
        let new_id = try_id(&self.variable_ids(), &variable.id);
        if new_id != variable.id {
            variable.id = new_id.clone();
        }
        new_id
    }

    fn version_line(&self) -> String {
        if self.version.is_some() {
            format!(
                "\n\n# BITSY VERSION {}.{}",
                self.version.as_ref().unwrap().major,
                self.version.as_ref().unwrap().minor
            )
        } else {
            "".to_string()
        }
    }

    fn room_format_line(&self) -> String {
        if self.room_format.is_some() {
            format!("\n\n! ROOM_FORMAT {}", self.room_format.unwrap())
        } else {
            "".to_string()
        }
    }

    fn font_line(&self) -> String {
        match self.font {
            Font::AsciiSmall => "".to_string(),
            Font::Custom => format!("\n\nDEFAULT_FONT {}", self.custom_font.as_ref().unwrap()),
            _ => format!("\n\nDEFAULT_FONT {}", self.font.to_string().unwrap()),
        }
    }

    fn text_direction_line(&self) -> &str {
        match self.text_direction {
            TextDirection::RightToLeft => "\n\nTEXT_DIRECTION RTL",
            _ => "",
        }
    }

    /// older bitsy games do not specify a version, but we can infer 1.0
    pub fn version(&self) -> Version {
        self.version.unwrap_or(Version { major: 1, minor: 0 })
    }

    /// older bitsy games do not specify a room format, but we can infer 0
    pub fn room_format(&self) -> RoomFormat {
        self.room_format.unwrap_or(RoomFormat::Contiguous)
    }
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

#[cfg(test)]
mod test {
    use super::*;
    use alloc::format;
    use alloc::string::ToString;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn game_from_string() {
        let output = Game::from(include_str!["test-resources/default.bitsy"]).unwrap();
        let expected = crate::mock::game_default();

        assert_eq!(output, expected);
    }

    #[test]
    fn game_to_string() {
        let output = crate::mock::game_default().to_string();
        let expected = include_str!["test-resources/default.bitsy"].to_string();
        assert_eq!(output, expected);
    }

    #[test]
    fn tile_ids() {
        assert_eq!(
            crate::mock::game_default().tile_ids(),
            vec!["a".to_string()]
        );
    }

    #[test]
    fn new_tile_id() {
        // default tile has an id of 10 ("a"), and 0 is reserved
        assert_eq!(crate::mock::game_default().new_tile_id(), "1".to_string());

        // for a game with a gap in the tile IDs, check the gap is used

        let mut game = crate::mock::game_default();
        let mut tiles: Vec<Tile> = Vec::new();

        // 0 is reserved; upper bound is non-inclusive
        for n in 1..10 {
            if n != 4 {
                let mut new_tile = crate::mock::tile_default();
                new_tile.id = format!("{}", n).to_string();
                tiles.push(new_tile);
            }
        }

        game.tiles = tiles;

        assert_eq!(game.new_tile_id(), "4".to_string());

        // fill in the space created above, then test that tile IDs get sorted

        let mut new_tile = crate::mock::tile_default();
        new_tile.id = "4".to_string();
        game.tiles.push(new_tile);

        assert_eq!(game.new_tile_id(), "a".to_string());
    }

    #[test]
    fn add_tile() {
        let mut game = crate::mock::game_default();
        let new_id = game.add_tile(crate::mock::tile_default());
        assert_eq!(new_id, "1".to_string());
        assert_eq!(game.tiles.len(), 2);
        let new_id = game.add_tile(crate::mock::tile_default());
        assert_eq!(new_id, "2".to_string());
        assert_eq!(game.tiles.len(), 3);
    }

    #[test]
    fn arabic() {
        let game = Game::from(include_str!("test-resources/arabic.bitsy")).unwrap();
        assert_eq!(game.font, Font::Arabic);
        assert_eq!(game.text_direction, TextDirection::RightToLeft);
    }

    #[test]
    fn version_formatting() {
        let mut game = crate::mock::game_default();
        game.version = Some(Version { major: 5, minor: 0 });
        assert!(game.to_string().contains("# BITSY VERSION 5.0"))
    }

    #[test]
    fn get_tiles_for_room() {
        assert_eq!(
            crate::mock::game_default().get_room_tiles("0"),
            vec![&crate::mock::tile_default()]
        )
    }

    #[test]
    fn add_item() {
        let mut game = crate::mock::game_default();
        game.add_item(crate::mock::item());
        game.add_item(crate::mock::item());

        let expected = vec![
            "0".to_string(),
            "1".to_string(),
            "6".to_string(),
            "2".to_string(),
        ];

        assert_eq!(game.item_ids(), expected);
    }

    #[test]
    fn find_tile_with_animation() {
        let game = crate::mock::game_default();
        let animation = vec![Image {
            pixels: vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1,
                1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1,
                1, 1, 1, 1, 1, 1, 1, 1,
            ],
        }];
        let output = game.find_tile_with_animation(&animation);
        let expected = Some(&game.tiles[0]);
        assert_eq!(output, expected);
    }

    #[test]
    fn empty_game_data_throws_error() {
        assert_eq!(
            Game::from("").unwrap_err(),
            crate::error::NotFound::Anything
        );
        assert_eq!(
            Game::from(" \n \r\n").unwrap_err(),
            crate::error::NotFound::Anything
        );
    }

    #[test]
    fn get_palette() {
        let mut game = crate::mock::game_default();
        let new_palette = Palette {
            id: "1".to_string(),
            name: Some("sadness".to_string()),
            colours: vec![
                Colour {
                    red: 133,
                    green: 131,
                    blue: 111,
                },
                Colour {
                    red: 105,
                    green: 93,
                    blue: 104,
                },
                Colour {
                    red: 62,
                    green: 74,
                    blue: 76,
                },
            ],
        };
        game.add_palette(new_palette.clone());
        assert_eq!(
            game.get_palette("0").unwrap(),
            &crate::mock::game_default().palettes[0]
        );
        assert_eq!(game.get_palette("1").unwrap(), &new_palette);
        assert_eq!(game.get_palette("2"), None);
    }

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
