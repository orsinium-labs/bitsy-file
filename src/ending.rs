use std::fmt;

// same as a dialogue basically
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ending {
    pub id: String,
    pub dialogue: String,
}

impl Ending {
    pub fn from_str(s: &str) -> Result<Self, crate::Error> {
        let lines: Vec<&str> = s.lines().collect();

        if lines.is_empty() || !lines[0].starts_with("END ") {
            return Err(crate::Error::Ending);
        }

        let id = lines[0].replace("END ", "");
        let dialogue = lines[1..].join("\n");

        Ok(Ending { id, dialogue })
    }
}

impl fmt::Display for Ending {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"END {}\n{}", self.id, self.dialogue)
    }
}

#[cfg(test)]
mod test {
    use crate::Ending;

    #[test]
    fn ending_from_string() {
        assert_eq!(
            Ending::from_str(include_str!("test-resources/ending")).unwrap(),
            Ending {
                id: "a".to_string(),
                dialogue: "This is a long line of dialogue. Blah blah blah".to_string()
            }
        );
    }

    #[test]
    fn ending_to_string() {
        assert_eq!(
            Ending {
                id: "7".to_string(),
                dialogue: "This is another long ending. So long, farewell, etc.".to_string()
            }.to_string(),
            "END 7\nThis is another long ending. So long, farewell, etc.".to_string()
        );
    }
}
