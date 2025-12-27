use crate::*;
use alloc::string::ToString;
use core::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Room {
    pub id: String,
    /// palette ID was optional in very early versions
    pub palette_id: Option<String>,
    pub name: Option<String>,
    /// tile IDs
    pub tiles: Vec<String>,
    pub items: Vec<Instance>,
    pub exits: Vec<ExitInstance>,
    pub endings: Vec<Instance>,
    /// old method of handling walls - a comma-separated list of tile IDs
    pub walls: Option<Vec<String>>,
}

impl Room {
    fn name_line(&self) -> String {
        optional_data_line("NAME", self.name.as_ref())
    }

    fn wall_line(&self) -> String {
        if let Some(walls) = &self.walls {
            optional_data_line("WAL", Some(walls.join(",")))
        } else {
            "".to_string()
        }
    }

    fn palette_line(&self) -> String {
        match &self.palette_id {
            Some(id) => optional_data_line("PAL", Some(id.clone())),
            None => "".to_string(),
        }
    }
}

impl From<&str> for Room {
    fn from(string: &str) -> Room {
        let string = string.replace("ROOM ", "");
        let string = string.replace("SET ", "");
        let mut lines = string.lines();
        let mut room = Room {
            id: lines.next().unwrap().to_string(),
            name: None,
            palette_id: None,
            items: Vec::new(),
            exits: Vec::new(),
            endings: Vec::new(),
            tiles: Vec::new(),
            walls: None,
        };

        // Read the first 16 lines (after ID) as tile IDs.
        const DIMENSION: usize = 16;
        for line in lines.by_ref().take(DIMENSION) {
            let comma_separated = line.contains(','); // old room format?
            let sep = if comma_separated { "," } else { "" };
            let mut line = line.split(sep);
            if !comma_separated {
                line.next();
            }
            for tile_id in line.take(DIMENSION) {
                room.tiles.push(tile_id.to_string());
            }
        }

        // After tiles, read the remaining room properties.
        for last_line in lines {
            let (first_word, _) = last_line.split_once(' ').unwrap_or_default();
            match first_word {
                "WAL" => {
                    let last_line = last_line.replace("WAL ", "");
                    let ids: Vec<&str> = last_line.split(',').collect();
                    room.walls = Some(ids.iter().map(|&id| id.to_string()).collect());
                }
                "NAME" => {
                    room.name = Some(last_line.replace("NAME ", "").to_string());
                }
                "PAL" => {
                    room.palette_id = Some(last_line.replace("PAL ", ""));
                }
                "ITM" => {
                    let last_line = last_line.replace("ITM ", "");
                    let (item_id, position) = last_line.split_once(' ').unwrap();
                    if let Ok(position) = Position::from_str(position) {
                        room.items.push(Instance {
                            position,
                            id: item_id.to_string(),
                        });
                    }
                }
                "EXT" => {
                    let last_line = last_line.replace("EXT ", "");
                    let parts: Vec<&str> = last_line.split(' ').collect();
                    let position = Position::from_str(parts[0]);

                    if let Ok(position) = position {
                        let exit = Exit::from_str(&format!("{} {}", parts[1], parts[2]));

                        if let Ok(exit) = exit {
                            let mut transition = None;
                            let mut dialogue_id = None;

                            let chunks = parts[3..].chunks(2);

                            for chunk in chunks {
                                if chunk[0] == "FX" {
                                    transition = Some(Transition::from_str(chunk[1]).unwrap());
                                } else if chunk[0] == "DLG" {
                                    dialogue_id = Some(chunk[1].to_string());
                                }
                            }

                            room.exits.push(ExitInstance {
                                position,
                                exit,
                                transition,
                                dialogue_id,
                            });
                        }
                    }
                }
                "END" => {
                    let last_line = last_line.replace("END ", "");
                    let (ending, position) = last_line.split_once(' ').unwrap();
                    let position = Position::from_str(position);
                    if let Ok(position) = position {
                        room.endings.push(Instance {
                            position,
                            id: ending.to_string(),
                        });
                    }
                }
                _ => {}
            }
        }
        room
    }
}

impl Room {
    pub fn to_string(&self, room_format: RoomFormat, room_type: RoomType) -> String {
        let mut tiles = String::new();
        let mut items = String::new();
        let mut exits = String::new();
        let mut endings = String::new();

        for line in self.tiles.chunks(16) {
            for tile in line {
                tiles.push_str(tile);
                if room_format == RoomFormat::CommaSeparated {
                    tiles.push(',');
                }
            }

            if room_format == RoomFormat::CommaSeparated {
                tiles.pop(); // remove trailing comma
            }

            tiles.push('\n');
        }

        tiles.pop(); // remove trailing newline

        for instance in &self.items {
            items.push_str(&format!("\nITM {} {}", instance.id, instance.position));
        }

        for instance in &self.exits {
            exits.push_str(&format!(
                "\nEXT {} {}{}{}{}",
                instance.position,
                instance.exit,
                match &instance.transition {
                    Some(transition) => transition,
                    None => &Transition::None,
                },
                if instance.dialogue_id.is_some() {
                    " DLG "
                } else {
                    ""
                },
                instance.dialogue_id.as_ref().unwrap_or(&"".to_string()),
            ));
        }

        for instance in &self.endings {
            endings.push_str(&format!("\nEND {} {}", instance.id, instance.position));
        }

        format!(
            "{} {}\n{}{}{}{}{}{}{}",
            room_type.to_string(),
            self.id,
            tiles,
            self.name_line(),
            self.wall_line(),
            items,
            exits,
            endings,
            self.palette_line()
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{Room, RoomFormat, RoomType};
    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn room_from_string() {
        assert_eq!(
            Room::from(include_str!("test-resources/room")),
            crate::mock::room()
        );
    }

    #[test]
    fn room_to_string() {
        assert_eq!(
            crate::mock::room().to_string(RoomFormat::CommaSeparated, RoomType::Room),
            include_str!("test-resources/room").to_string()
        );
    }

    #[test]
    fn room_walls_array() {
        let output = Room::from(include_str!("test-resources/room-with-walls"));

        assert_eq!(output.walls, Some(vec!["a".to_string(), "f".to_string()]));
    }
}
