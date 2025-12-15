use crate::{
    optional_data_line, Exit, ExitInstance, Instance, Position, RoomFormat, RoomType, Transition,
};

use std::collections::HashMap;

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

impl From<String> for Room {
    fn from(string: String) -> Room {
        let string = string.replace("ROOM ", "");
        let string = string.replace("SET ", "");
        let mut lines: Vec<&str> = string.lines().collect();
        let id = lines[0].to_string();
        let mut name = None;
        let mut palette_id = None;
        let mut items: Vec<Instance> = Vec::new();
        let mut exits: Vec<ExitInstance> = Vec::new();
        let mut endings: Vec<Instance> = Vec::new();
        let mut walls = None;

        loop {
            let last_line = lines.pop().unwrap();

            if last_line.starts_with("WAL") {
                let last_line = last_line.replace("WAL ", "");
                let ids: Vec<&str> = last_line.split(',').collect();
                walls = Some(ids.iter().map(|&id| id.to_string()).collect());
            } else if last_line.starts_with("NAME") {
                name = Some(last_line.replace("NAME ", "").to_string());
            } else if last_line.starts_with("PAL") {
                palette_id = Some(last_line.replace("PAL ", ""));
            } else if last_line.starts_with("ITM") {
                let last_line = last_line.replace("ITM ", "");
                let item_position: Vec<&str> = last_line.split(' ').collect();
                let item_id = item_position[0];
                let position = item_position[1];

                if let Ok(position) = Position::from_str(position) {
                    items.push(Instance {
                        position,
                        id: item_id.to_string(),
                    });
                }
            } else if last_line.starts_with("EXT") {
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

                        exits.push(ExitInstance {
                            position,
                            exit,
                            transition,
                            dialogue_id,
                        });
                    }
                }
            } else if last_line.starts_with("END") {
                let last_line = last_line.replace("END ", "");
                let ending_position: Vec<&str> = last_line.split(' ').collect();
                let ending = ending_position[0].to_string();
                let position = ending_position[1];
                let position = Position::from_str(position);

                if let Ok(position) = position {
                    endings.push(Instance {
                        position,
                        id: ending,
                    });
                }
            } else {
                lines.push(last_line);
                break;
            }
        }

        let lines = &lines[1..];
        let dimension = lines.len(); // x or y, e.g. `16` for 16x16
        let mut tiles: Vec<String> = Vec::new();

        for line in lines.iter() {
            let comma_separated = line.contains(','); // old room format?
            let mut line: Vec<&str> = line.split(if comma_separated { "," } else { "" }).collect();

            if !comma_separated {
                line = line[1..].to_owned();
            }
            let line = line[..dimension].to_owned();

            for tile_id in line {
                tiles.push(tile_id.to_string());
            }
        }

        items.reverse();
        exits.reverse();
        endings.reverse();

        Room {
            id,
            palette_id,
            name,
            tiles,
            items,
            exits,
            endings,
            walls,
        }
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
            items.push_str(&format!(
                "\nITM {} {}",
                instance.id,
                instance.position.to_string()
            ));
        }

        for instance in &self.exits {
            exits.push_str(&format!(
                "\nEXT {} {}{}{}{}",
                instance.position.to_string(),
                instance.exit.to_string(),
                match &instance.transition {
                    Some(transition) => transition,
                    None => &Transition::None,
                }
                .to_string(),
                if instance.dialogue_id.is_some() {
                    " DLG "
                } else {
                    ""
                },
                instance.dialogue_id.as_ref().unwrap_or(&"".to_string()),
            ));
        }

        for instance in &self.endings {
            endings.push_str(&format!(
                "\nEND {} {}",
                instance.id,
                instance.position.to_string()
            ));
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

    /// "changes" is a hash of old -> new tile IDs
    pub fn change_tile_ids(&mut self, changes: &HashMap<String, String>) {
        self.tiles = self
            .tiles
            .iter()
            .map(|tile_id| changes.get(tile_id).unwrap_or(tile_id).clone())
            .collect();
    }
}

#[cfg(test)]
mod test {
    use crate::{Room, RoomFormat, RoomType};

    #[test]
    fn room_from_string() {
        assert_eq!(
            Room::from(include_str!("test-resources/room").to_string()),
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
        let output = Room::from(include_str!("test-resources/room-with-walls").to_string());

        assert_eq!(output.walls, Some(vec!["a".to_string(), "f".to_string()]));
    }
}
