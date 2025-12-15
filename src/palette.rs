use crate::Colour;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Palette {
    pub id: String,
    pub name: Option<String>,
    pub colours: Vec<Colour>,
}

impl Palette {
    pub fn from_str(s: &str) -> Result<(Palette, Vec<crate::Error>), crate::Error> {
        let mut lines: Vec<&str> = s.lines().collect();

        if lines.is_empty() {
            return Err(crate::Error::Palette);
        }

        let mut id = String::new();
        let mut name = None;
        let mut colours = Vec::new();
        let mut warnings = Vec::new();

        while !lines.is_empty() {
            let line = lines.pop().unwrap();

            if line.starts_with("PAL ") {
                id = line.replace("PAL ", "");
            } else if line.starts_with("NAME ") {
                name = Some(line.replace("NAME ", ""));
            } else {
                let result = Colour::from(line);
                if let Ok(colour) = result {
                    colours.push(colour)
                } else {
                    warnings.push(result.unwrap_err());
                }
            }
        }

        colours.reverse();

        Ok((Palette { id, name, colours }, warnings))
    }
}

impl ToString for Palette {
    fn to_string(&self) -> String {
        let name = if self.name.as_ref().is_some() {
            format!("NAME {}\n", self.name.as_ref().unwrap())
        } else {
            "".to_string()
        };

        let mut colours = String::new();
        for colour in &self.colours {
            colours.push_str(&format!("{}\n", colour.to_string()));
        }
        colours.pop();

        format!("PAL {}\n{}{}", self.id, name, colours)
    }
}

#[cfg(test)]
mod test {
    use crate::{Colour, Palette};

    #[test]
    fn palette_from_string() {
        let (output, _) = Palette::from_str("PAL 1\nNAME lamplight\n45,45,59\n66,60,39\n140,94,1").unwrap();

        let expected = Palette {
            id: "1".to_string(),
            name: Some("lamplight".to_string()),
            colours: vec![
                Colour {
                    red: 45,
                    green: 45,
                    blue: 59,
                },
                Colour {
                    red: 66,
                    green: 60,
                    blue: 39,
                },
                Colour {
                    red: 140,
                    green: 94,
                    blue: 1,
                },
            ],
        };

        assert_eq!(output, expected);
    }

    #[test]
    fn palette_from_string_no_name() {
        let (output, _) = Palette::from_str("PAL 9\n45,45,59\n66,60,39\n140,94,1").unwrap();

        let expected = Palette {
            id: "9".to_string(),
            name: None,
            colours: vec![
                Colour {
                    red: 45,
                    green: 45,
                    blue: 59,
                },
                Colour {
                    red: 66,
                    green: 60,
                    blue: 39,
                },
                Colour {
                    red: 140,
                    green: 94,
                    blue: 1,
                },
            ],
        };

        assert_eq!(output, expected);
    }

    #[test]
    fn palette_to_string() {
        let output = Palette {
            id: "g".to_string(),
            name: Some("moss".to_string()),
            colours: vec![
                Colour {
                    red: 1,
                    green: 2,
                    blue: 3,
                },
                Colour {
                    red: 255,
                    green: 254,
                    blue: 253,
                },
                Colour {
                    red: 126,
                    green: 127,
                    blue: 128,
                },
            ],
        }.to_string();

        let expected = "PAL g\nNAME moss\n1,2,3\n255,254,253\n126,127,128";
        assert_eq!(output, expected);
    }
}
