#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Font {
    AsciiSmall, // default - does not appear in game data
    UnicodeEuropeanSmall,
    UnicodeEuropeanLarge,
    UnicodeAsian,
    Arabic,
    Custom,
}

impl Font {
    pub(crate) fn from(str: &str) -> Font {
        match str {
            "unicode_european_small" => Font::UnicodeEuropeanSmall,
            "unicode_european_large" => Font::UnicodeEuropeanLarge,
            "unicode_asian"          => Font::UnicodeAsian,
            "arabic"                 => Font::Arabic,
            _                        => Font::Custom,
        }
    }

    pub(crate) fn to_string(&self) -> Result<String, crate::Error> {
        match &self {
            Font::UnicodeEuropeanSmall => Ok("unicode_european_small".to_string()),
            Font::UnicodeEuropeanLarge => Ok("unicode_european_large".to_string()),
            Font::UnicodeAsian         => Ok("unicode_asian".to_string()),
            Font::Arabic               => Ok("arabic".to_string()),
            _                          => Err(crate::Error::Font),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TextDirection {
    LeftToRight, // default
    RightToLeft,
}
