use crate::Position;
use alloc::string::ToString;
use alloc::{string::String, vec::Vec};
use core::fmt;
use core::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Transition {
    None,
    FadeToWhite,
    FadeToBlack,
    Wave,
    Tunnel,
    SlideUp,
    SlideDown,
    SlideLeft,
    SlideRight,
}

impl FromStr for Transition {
    type Err = crate::Error;

    fn from_str(str: &str) -> Result<Transition, Self::Err> {
        match str {
            "fade_w" => Ok(Transition::FadeToWhite),
            "fade_b" => Ok(Transition::FadeToBlack),
            "wave" => Ok(Transition::Wave),
            "tunnel" => Ok(Transition::Tunnel),
            "slide_u" => Ok(Transition::SlideUp),
            "slide_d" => Ok(Transition::SlideDown),
            "slide_l" => Ok(Transition::SlideLeft),
            "slide_r" => Ok(Transition::SlideRight),
            _ => Err(crate::Error::Transition),
        }
    }
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Transition::FadeToWhite => " FX fade_w",
                Transition::FadeToBlack => " FX fade_b",
                Transition::Wave => " FX wave",
                Transition::Tunnel => " FX tunnel",
                Transition::SlideUp => " FX slide_u",
                Transition::SlideDown => " FX slide_d",
                Transition::SlideLeft => " FX slide_l",
                Transition::SlideRight => " FX slide_r",
                Transition::None => "",
            }
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Exit {
    /// destination
    pub room_id: String,
    /// id
    pub position: Position,
    pub effect: Transition,
}

impl FromStr for Exit {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() < 2 {
            return Err(crate::Error::Exit);
        }

        let mut parts = parts.iter();

        let room_id = parts.next().unwrap().to_string();
        let position = Position::from_str(parts.next().unwrap())?;

        let effect = if parts.next().is_some() {
            Transition::from_str(parts.next().unwrap())?
        } else {
            Transition::None
        };

        Ok(Exit {
            room_id,
            position,
            effect,
        })
    }
}

impl fmt::Display for Exit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}{}", self.room_id, self.position, self.effect)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exit_from_string() {
        assert_eq!(
            Exit::from_str("a 12,13").unwrap(),
            Exit {
                room_id: "a".to_string(),
                position: Position { x: 12, y: 13 },
                effect: Transition::None
            }
        );
    }

    #[test]
    fn exit_from_string_with_fx() {
        assert_eq!(
            Exit::from_str("a 12,13 FX slide_u").unwrap(),
            Exit {
                room_id: "a".to_string(),
                position: Position { x: 12, y: 13 },
                effect: Transition::SlideUp
            }
        );
    }

    #[test]
    fn exit_to_string() {
        assert_eq!(
            Exit {
                room_id: "8".to_string(),
                position: Position { x: 5, y: 6 },
                effect: Transition::None
            }
            .to_string(),
            "8 5,6".to_string()
        );
    }

    #[test]
    fn exit_to_string_with_fx() {
        assert_eq!(
            Exit {
                room_id: "8".to_string(),
                position: Position { x: 5, y: 6 },
                effect: Transition::FadeToWhite
            }
            .to_string(),
            "8 5,6 FX fade_w".to_string()
        );
    }
}
