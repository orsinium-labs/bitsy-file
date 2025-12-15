use core::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn from_str(s: &str) -> Result<Self, crate::Error> {
        let mut parts = s.split(',');

        let x = parts.next().unwrap();
        let y = parts.next().unwrap();

        if let (Ok(x), Ok(y)) = (x.parse(), y.parse()) {
            Ok(Position { x, y })
        } else {
            Err(crate::Error::Position)
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use crate::Position;

    #[test]
    fn position_from_str() {
        assert_eq!(
            Position::from_str("4,12").unwrap(),
            Position { x: 4, y: 12 }
        );
    }

    #[test]
    fn position_from_malformed_str() {
        assert!(Position::from_str("14,-1").is_err())
    }

    #[test]
    fn position_to_string() {
        assert_eq!(Position { x: 4, y: 12 }.to_string(), "4,12".to_string())
    }
}
