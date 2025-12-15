use core::fmt;

#[derive(Debug, PartialEq)]
pub enum NotFound {
    Anything,
    Avatar,
    Room,
    Sprite,
    Tile,
}

impl NotFound {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotFound::Anything => "file is empty",
            NotFound::Avatar => "avatar not found",
            NotFound::Room => "room not found",
            NotFound::Sprite => "sprite not found",
            NotFound::Tile => "tile not found",
        }
    }
}

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
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

impl Error {
    pub fn as_str(&self) -> &'static str {
        match self {
            Error::Colour => "color error",
            Error::Dialogue => "dialog error",
            Error::Ending => "ending error",
            Error::Exit => "exit error",
            Error::Font => "font error",
            Error::Game { missing } => missing.as_str(),
            Error::Image => "image error",
            Error::Item => "item error",
            Error::Palette => "palette error",
            Error::Position => "position error",
            Error::Room => "room error",
            Error::Sprite => "sprite error",
            Error::Text => "text error",
            Error::Tile => "tile error",
            Error::Transition => "transition error",
            Error::Variable => "variable error",
            Error::Version => "version error",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::error::Error for Error {}
