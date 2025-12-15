use crate::optional_data_line;
use alloc::{string::String, vec::Vec};
use core::fmt;
use core::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dialogue {
    pub id: String,
    pub contents: String,
    pub name: Option<String>,
}

impl FromStr for Dialogue {
    type Err = crate::Error;

    fn from_str(str: &str) -> Result<Dialogue, Self::Err> {
        let mut lines: Vec<&str> = str.lines().collect();

        if lines.is_empty() || !lines[0].starts_with("DLG ") {
            return Err(crate::Error::Dialogue);
        }

        let id = lines[0].replace("DLG ", "");

        let last_line = lines.pop().unwrap();

        let name = if last_line.starts_with("NAME ") {
            Some(last_line.replace("NAME ", ""))
        } else {
            lines.push(last_line);
            None
        };

        let contents = lines[1..].join("\n");

        Ok(Dialogue { id, contents, name })
    }
}

impl fmt::Display for Dialogue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DLG {}\n{}{}",
            self.id,
            self.contents,
            optional_data_line("NAME", self.name.as_ref())
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn dialogue_from_str() {
        let output =
            Dialogue::from_str("DLG h\nhello\nNAME not a dialogue name\nNAME a dialogue name")
                .unwrap();

        let expected = Dialogue {
            id: "h".to_string(),
            contents: "hello\nNAME not a dialogue name".to_string(),
            name: Some("a dialogue name".to_string()),
        };

        assert_eq!(output, expected);
    }

    #[test]
    fn dialogue_to_string() {
        let output = Dialogue {
            id: "y".to_string(),
            contents: "This is a bit of dialogue,\nblah blah\nblah blah".to_string(),
            name: Some("a dialogue name".to_string()),
        }
        .to_string();

        let expected =
            "DLG y\nThis is a bit of dialogue,\nblah blah\nblah blah\nNAME a dialogue name";

        assert_eq!(output, expected);
    }
}
