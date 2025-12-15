use crate::error::NotFound;
use crate::*;
use alloc::borrow::ToOwned;
use alloc::string::ToString;
use alloc::{format, string::String, vec::Vec};
use core::borrow::BorrowMut;
use core::fmt;
use core::str::FromStr;
use std::collections::HashMap;

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
    fn from(str: &str) -> Result<RoomFormat, InvalidRoomFormat> {
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
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum RoomType {
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
    fn from(str: &str) -> Result<Version, VersionError> {
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

#[derive(Clone, Debug, PartialEq)]
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
}

impl Game {
    pub fn from(string: String) -> Result<(Game, Vec<crate::Error>), crate::error::NotFound> {
        if string.trim() == "" {
            return Err(crate::error::NotFound::Anything);
        }
        let string = string.replace("\r\n", "\n");
        let string = string.trim_start_matches('\n').to_string();
        let mut segments = crate::segments_from_str(&string);

        let mut name = "".to_string();

        // game names can be empty - so when we strip out the leading whitespace above,
        // it means that the first segment might not be the game name.
        // so, check if the first segment is actually the next segment of game data
        // to avoid setting the game name to "# BITSY VERSION 7.0" or something
        if segments[0].starts_with("\"\"\"") // multi-line game name
            ||
            (
                ! segments[0].starts_with("# BITSY VERSION ")
                &&
                ! segments[0].starts_with("! ROOM_FORMAT ")
                &&
                ! segments[0].starts_with("PAL ")
                &&
                ! segments[0].starts_with("DEFAULT_FONT ")
                &&
                ! segments[0].starts_with("TEXT_DIRECTION ")
            )
        {
            name = segments[0].to_string();
            segments = segments[1..].to_owned();
        }

        let segments = segments;

        let name = name;
        let mut dialogues: Vec<Dialogue> = Vec::new();
        let mut endings: Vec<Ending> = Vec::new();
        let mut variables: Vec<Variable> = Vec::new();
        let mut font_data: Option<String> = None;

        let mut version = None;
        let mut room_format = None;
        let mut room_type = RoomType::Room;
        let mut font = Font::AsciiSmall;
        let mut custom_font = None;
        let mut text_direction = TextDirection::LeftToRight;
        let mut palettes: Vec<Palette> = Vec::new();
        let mut rooms: Vec<Room> = Vec::new();
        let mut tiles: Vec<Tile> = Vec::new();
        let mut sprites: Vec<Sprite> = Vec::new();
        let mut items: Vec<Item> = Vec::new();
        let mut avatar_exists = false;

        let mut warnings = Vec::new();
        for segment in segments {
            if segment.starts_with("# BITSY VERSION") {
                let segment = segment.replace("# BITSY VERSION ", "");
                let result = Version::from(&segment);

                if let Ok(v) = result {
                    version = Some(v);
                } else {
                    warnings.push(Error::Version);
                }
            } else if segment.starts_with("! ROOM_FORMAT") {
                let segment = segment.replace("! ROOM_FORMAT ", "");
                room_format =
                    Some(RoomFormat::from(&segment).unwrap_or(RoomFormat::CommaSeparated));
            } else if segment.starts_with("DEFAULT_FONT") {
                let segment = segment.replace("DEFAULT_FONT ", "");

                font = Font::from(&segment);

                if font == Font::Custom {
                    custom_font = Some(segment.to_string());
                }
            } else if segment.trim() == "TEXT_DIRECTION RTL" {
                text_direction = TextDirection::RightToLeft;
            } else if segment.starts_with("PAL ") {
                let result = Palette::from_str(&segment);
                if let Ok((palette, mut errors)) = result {
                    palettes.push(palette);
                    warnings.append(&mut errors);
                } else {
                    warnings.push(result.unwrap_err());
                }
            } else if segment.starts_with("ROOM ") || segment.starts_with("SET ") {
                if segment.starts_with("SET ") {
                    room_type = RoomType::Set;
                }
                rooms.push(Room::from(segment));
            } else if segment.starts_with("TIL ") {
                tiles.push(Tile::from(segment));
            } else if segment.starts_with("SPR ") {
                let result = Sprite::from_str(&segment);

                if let Ok(sprite) = result {
                    avatar_exists |= sprite.id == "A";

                    sprites.push(sprite);
                } else {
                    warnings.push(result.unwrap_err());
                }
            } else if segment.starts_with("ITM ") {
                let result = Item::from_str(&segment);

                if let Ok(item) = result {
                    items.push(item);
                } else {
                    warnings.push(result.unwrap_err());
                }
            } else if segment.starts_with("DLG ") {
                let result = Dialogue::from_str(&segment);

                if let Ok(dialogue) = result {
                    dialogues.push(dialogue);
                } else {
                    warnings.push(result.unwrap_err());
                }
            } else if segment.starts_with("END ") {
                let result = Ending::from_str(&segment);

                if let Ok(ending) = result {
                    endings.push(ending);
                } else {
                    warnings.push(result.unwrap_err());
                }
            } else if segment.starts_with("VAR ") {
                variables.push(Variable::from(segment));
            } else if segment.starts_with("FONT ") {
                font_data = Some(segment);
            }
        }

        if !avatar_exists {
            warnings.push(crate::Error::Game {
                missing: NotFound::Avatar,
            });
        }

        Ok((
            Game {
                name,
                version,
                room_format,
                room_type,
                font,
                custom_font,
                text_direction,
                palettes,
                rooms,
                tiles,
                sprites,
                items,
                dialogues,
                endings,
                variables,
                font_data,
            },
            warnings,
        ))
    }

    /// todo refactor this into "get T by ID", taking a Vec<T> and an ID name?
    pub fn get_sprite_by_id(&self, id: String) -> Result<&Sprite, crate::error::NotFound> {
        let index = self.sprites.iter().position(|sprite| sprite.id == id);

        match index {
            Some(index) => Ok(&self.sprites[index]),
            None => Err(crate::error::NotFound::Sprite),
        }
    }

    pub fn get_tile_by_id(&self, id: String) -> Result<&Tile, crate::error::NotFound> {
        let index = self.tiles.iter().position(|tile| tile.id == id);

        match index {
            Some(index) => Ok(&self.tiles[index]),
            None => Err(crate::error::NotFound::Tile),
        }
    }

    pub fn get_room_by_id(&self, id: String) -> Result<&Room, crate::error::NotFound> {
        let index = self.rooms.iter().position(|room| room.id == id);

        match index {
            Some(index) => Ok(&self.rooms[index]),
            None => Err(crate::error::NotFound::Room),
        }
    }

    pub fn get_avatar(&self) -> Result<&Sprite, crate::error::NotFound> {
        self.get_sprite_by_id("A".to_string())
    }

    // todo result
    pub fn get_tiles_by_ids(&self, ids: Vec<String>) -> Vec<&Tile> {
        let mut tiles: Vec<&Tile> = Vec::new();

        for id in ids {
            if let Ok(tile) = self.get_tile_by_id(id) {
                tiles.push(tile);
            }
        }

        tiles
    }

    pub fn get_tiles_for_room(&self, id: String) -> Result<Vec<&Tile>, crate::error::NotFound> {
        let room = self.get_room_by_id(id)?;
        let mut tile_ids = room.tiles.clone();
        tile_ids.sort();
        tile_ids.dedup();
        // remove 0 as this isn't a real tile
        let zero_index = tile_ids.iter().position(|i| i == "0");
        if let Some(zero_index) = zero_index {
            tile_ids.remove(zero_index);
        }
        // remove Ok once this function returns a result
        Ok(self.get_tiles_by_ids(tile_ids))
    }

    // return? array of changes made? error/ok?
    pub fn merge(&mut self, game: &Game) {
        // ignore title, version, room format, room type, font, text direction

        let mut palette_id_changes: HashMap<String, String> = HashMap::new();
        let mut tile_id_changes: HashMap<String, String> = HashMap::new();
        let mut dialogue_id_changes: HashMap<String, String> = HashMap::new();
        let mut ending_id_changes: HashMap<String, String> = HashMap::new();
        let mut item_id_changes: HashMap<String, String> = HashMap::new();
        let mut room_id_changes: HashMap<String, String> = HashMap::new();
        let mut sprite_id_changes: HashMap<String, String> = HashMap::new();

        fn insert_if_different(map: &mut HashMap<String, String>, old: String, new: String) {
            if old != new && !map.contains_key(&old) {
                map.insert(old, new);
            }
        }

        // alternatively - instead of handling these types in a specific order,
        // we could calculate the new IDs for each type first,
        // then handle the sections one by one

        // a room has a palette, so handle palettes before rooms
        for palette in &game.palettes {
            insert_if_different(
                palette_id_changes.borrow_mut(),
                palette.id.clone(),
                self.add_palette(palette.clone()),
            );
        }

        // a room has tiles, so handle before room
        for tile in &game.tiles {
            insert_if_different(
                tile_id_changes.borrow_mut(),
                tile.id.clone(),
                self.add_tile(tile.clone()),
            );
        }

        for variable in &game.variables {
            // don't change ID - just avoid duplicates
            if !self.variable_ids().contains(&variable.id) {
                self.add_variable(variable.clone());
            }
        }

        for item in &game.items {
            let old_id = item.id.clone();
            let new_id = try_id(&self.item_ids(), &item.id);
            insert_if_different(item_id_changes.borrow_mut(), old_id, new_id)
        }

        // a sprite has a dialogue, so handle before sprites
        // dialogue can have variables, so handle before after variables
        for dialogue in &game.dialogues {
            let mut dialogue = dialogue.clone();

            for (old, new) in &item_id_changes {
                // todo is there a better way of doing this?
                dialogue.contents = dialogue
                    .contents
                    .replace(&format!("item \"{}\"", old), &format!("item \"{}\"", new));
            }

            let old_id = dialogue.id.clone();
            let new_id = self.add_dialogue(dialogue);
            insert_if_different(dialogue_id_changes.borrow_mut(), old_id, new_id);
        }

        // an ending lives in a room, so handle endings before rooms
        for ending in &game.endings {
            insert_if_different(
                ending_id_changes.borrow_mut(),
                ending.id.clone(),
                self.add_ending(ending.clone()),
            );
        }

        // an item has a dialogue ID, so we need to handle these after dialogues
        // an item instance lives in a room so these must be handled before rooms
        for item in &game.items {
            let mut item = item.clone();

            if item_id_changes.contains_key(&item.id) {
                item.id = item_id_changes[&item.id].clone();
            }

            if let Some(key) = item.dialogue_id.clone() {
                if let Some(change) = dialogue_id_changes.get(&key) {
                    item.dialogue_id = Some(change.clone());
                }
            }

            self.add_item(item);
        }

        // calculate all of the new room IDs first
        // to insert any new room, we need to know the new IDs of every room
        // to maintain the integrity of exits and endings

        let mut all_room_ids = self.room_ids();

        for room in &game.rooms {
            let old = room.id.clone();
            let new = try_id(&all_room_ids, &room.id);
            insert_if_different(room_id_changes.borrow_mut(), old, new.clone());
            all_room_ids.push(new);
        }

        // needs to be handled after palettes, tiles, items, exits, endings
        // and before sprites
        for room in &game.rooms {
            let mut room = room.clone();

            if let Some(room_id_change) = room_id_changes.get(&room.id) {
                room.id = room_id_change.clone();
            }

            if let Some(key) = room.palette_id.clone() {
                if let Some(change) = palette_id_changes.get(&key) {
                    room.palette_id = Some(change.clone());
                }
            }

            room.change_tile_ids(&tile_id_changes);

            room.items = room
                .items
                .iter()
                .map(|instance| {
                    if item_id_changes.contains_key(&instance.id) {
                        Instance {
                            position: instance.position.clone(),
                            id: item_id_changes[&instance.id].clone(),
                        }
                    } else {
                        instance.clone()
                    }
                })
                .collect();

            room.exits = room
                .exits
                .iter()
                .map(|exit| {
                    let mut exit = exit.clone();

                    let key = exit.exit.room_id.clone();

                    if let Some(change) = room_id_changes.get(&key) {
                        exit.exit.room_id = change.clone();
                    }

                    if let Some(key) = exit.dialogue_id.clone() {
                        if let Some(dialogue_change) = dialogue_id_changes.get(&key) {
                            exit.dialogue_id = Some(dialogue_change.clone());
                        }
                    }

                    exit
                })
                .collect();

            room.endings = room
                .endings
                .iter()
                .map(|ending| {
                    let mut ending = ending.clone();
                    let key = ending.id.clone();

                    if let Some(change) = ending_id_changes.get(&key) {
                        ending.id = change.clone();
                    }

                    ending
                })
                .collect();

            self.add_room(room);
        }

        // a sprite has a dialogue ID, so we need to handle these after dialogues
        // a sprite has a position in a room, so we need to handle these after the rooms
        for sprite in &game.sprites {
            let mut sprite = sprite.clone();
            // avoid having two avatars
            if sprite.id == "A" {
                sprite.id = "0".to_string(); // just a default value for replacement
            }

            if let Some(key) = sprite.dialogue_id.clone() {
                if dialogue_id_changes.contains_key(&key) {
                    sprite.dialogue_id = Some(dialogue_id_changes[&key].clone());
                }
            }

            if let Some(key) = sprite.room_id.clone() {
                if let Some(change) = room_id_changes.get(&key) {
                    sprite.room_id = Some(change.clone());
                }
            }

            let old_id = sprite.id.clone();
            let new_id = self.add_sprite(sprite);
            insert_if_different(sprite_id_changes.borrow_mut(), old_id, new_id);
        }
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

    /// todo I think I need a generic `dedupe(&mut self, Vec<T>)` function
    /// it would have to take a closure for comparing a given T (see the background_tile below)
    /// and a closure for what to do with the changed IDs
    pub fn dedupe_tiles(&mut self) {
        let mut tiles_temp = self.tiles.clone();
        let mut unique_tiles: Vec<Tile> = Vec::new();
        let mut tile_id_changes: HashMap<String, String> = HashMap::new();

        while let Some(tile) = tiles_temp.pop() {
            if tile == crate::mock::tile_background() {
                tile_id_changes.insert(tile.id, "0".to_string());
            } else if tiles_temp.contains(&tile) {
                tile_id_changes.insert(tile.id.clone(), self.get_tile_id(&tile).unwrap());
            } else {
                unique_tiles.push(tile);
            }
        }

        for room in &mut self.rooms {
            room.change_tile_ids(&tile_id_changes);
        }

        unique_tiles.reverse();

        self.tiles = unique_tiles;
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

#[cfg(test)]
mod test {
    use crate::{Colour, Font, Game, Image, Palette, TextDirection, Tile, Version};
    use alloc::format;
    use alloc::string::ToString;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn game_from_string() {
        let (output, _) =
            Game::from(include_str!["test-resources/default.bitsy"].to_string()).unwrap();
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
        let (game, _) =
            Game::from(include_str!("test-resources/arabic.bitsy").to_string()).unwrap();

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
            crate::mock::game_default()
                .get_tiles_for_room("0".to_string())
                .unwrap(),
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
    fn merge() {
        // try merging two default games
        let mut game = crate::mock::game_default();
        game.merge(&crate::mock::game_default());

        assert_eq!(game.room_ids(), vec!["0".to_string(), "1".to_string()]);
        assert_eq!(game.tile_ids(), vec!["a".to_string(), "1".to_string()]); // 0 is reserved
                                                                             // duplicate avatar (SPR A) gets converted into a normal sprite
        assert_eq!(
            game.sprite_ids(),
            vec![
                "A".to_string(),
                "a".to_string(),
                "0".to_string(),
                "1".to_string()
            ]
        );
        assert_eq!(
            game.item_ids(),
            vec![
                "0".to_string(),
                "1".to_string(),
                "2".to_string(),
                "3".to_string()
            ]
        );
        assert_eq!(
            game.dialogue_ids(),
            vec![
                "0".to_string(),
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string()
            ]
        );
        assert_eq!(game.palette_ids(), vec!["0".to_string(), "1".to_string()]);
        assert_eq!(
            game.get_room_by_id("1".to_string()).unwrap().palette_id,
            Some("1".to_string())
        );

        // test sprites in non-zero rooms in merged game
        let mut game_a = crate::mock::game_default();
        let mut game_b = crate::mock::game_default();
        let mut room = crate::mock::room();
        let mut sprite = crate::mock::sprite();
        let room_id = "2".to_string();
        room.id = room_id.clone();
        sprite.room_id = Some(room_id.clone());
        game_b.add_sprite(sprite);
        game_a.merge(&game_b);
        assert_eq!(
            game_a.get_sprite_by_id("2".to_string()).unwrap().room_id,
            Some(room_id)
        );
    }

    #[test]
    fn dedupe_tiles() {
        let mut game = crate::mock::game_default();
        game.add_tile(crate::mock::tile_default());
        game.add_tile(crate::mock::tile_default());
        game.add_tile(crate::mock::tile_background());
        game.dedupe_tiles();
        assert_eq!(game.tiles, vec![crate::mock::tile_default()]);

        let tile_a = Tile {
            id: "0".to_string(),
            name: Some("apple".to_string()),
            wall: Some(true),
            animation_frames: vec![Image {
                pixels: vec![
                    0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0,
                    1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1,
                    1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1,
                ],
            }],
            colour_id: Some(1),
        };

        let tile_b = Tile {
            id: "1".to_string(),
            name: Some("frogspawn".to_string()),
            wall: Some(false),
            animation_frames: vec![Image {
                pixels: vec![
                    1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1,
                    0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0,
                    1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1,
                ],
            }],
            colour_id: None,
        };

        game.add_tile(tile_a.clone());
        game.add_tile(tile_b.clone());
        game.add_tile(tile_a.clone());
        game.add_tile(tile_b.clone());

        game.dedupe_tiles();

        assert_eq!(
            game.tiles,
            vec![crate::mock::tile_default(), tile_a, tile_b]
        );
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
            Game::from("".to_string()).unwrap_err(),
            crate::error::NotFound::Anything
        );
        assert_eq!(
            Game::from(" \n \r\n".to_string()).unwrap_err(),
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
}
