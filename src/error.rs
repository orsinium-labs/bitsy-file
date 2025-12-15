use core::fmt;

#[derive(Debug, PartialEq)]
pub enum NotFound {
    Anything,
    Avatar,
    Room,
    Sprite,
    Tile,
}

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Not found: {} data",
            match self {
                NotFound::Anything => "game",
                NotFound::Avatar => "avatar",
                NotFound::Room => "room",
                NotFound::Sprite => "sprite",
                NotFound::Tile => "tile",
            }
        )
    }
}

#[derive(Debug)]
pub enum Error {
    Colour,
    Dialogue,
    Ending,
    Exit,
    Font,
    Game { missing: NotFound },
    Image,
    Item,
    Palette,
    Position,
    Room,
    Sprite,
    Text,
    Tile,
    Transition,
    Variable,
    Version,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl core::error::Error for Error {}
