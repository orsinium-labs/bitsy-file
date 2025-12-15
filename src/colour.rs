#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Colour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Colour {
    pub fn from(string: &str) -> Result<Colour, crate::Error> {
        let values: Vec<&str> = string.trim_matches(',').split(',').collect();

        if values.len() != 3 {
            return Err(crate::Error::Colour);
        }

        let red: u8 = values[0].parse().unwrap_or(0);
        let green: u8 = values[1].parse().unwrap_or(0);
        let blue: u8 = values[2].parse().unwrap_or(0);

        Ok(Colour { red, green, blue })
    }

    pub fn from_hex(hex: &str) -> Result<Colour, crate::Error> {
        let hex = hex.to_lowercase().trim_start_matches('#').to_string();
        Ok(Colour {
            red: u8::from_str_radix(&hex[..2], 16).unwrap(),
            green: u8::from_str_radix(&hex[2..4], 16).unwrap(),
            blue: u8::from_str_radix(&hex[4..6], 16).unwrap(),
        })
    }
}

impl ToString for Colour {
    fn to_string(&self) -> String {
        format!("{},{},{}", self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod test {
    use crate::Colour;

    #[test]
    fn colour_from_string() {
        assert_eq!(
            Colour::from("0,255,0").unwrap(),
            Colour {
                red: 0,
                green: 255,
                blue: 0
            }
        );
    }

    #[test]
    fn colour_to_string() {
        assert_eq!(
            Colour {
                red: 22,
                green: 33,
                blue: 44,
            }
            .to_string(),
            "22,33,44".to_string()
        );
    }

    #[test]
    fn colour_missing_value() {
        assert!(Colour::from("0,0").is_err());
    }

    #[test]
    fn colour_ambiguous_value() {
        assert!(Colour::from("0,0,").is_err());
    }

    #[test]
    fn colour_extraneous_value() {
        assert!(Colour::from("0,0,0,0").is_err());
    }

    #[test]
    fn colour_from_hex() {
        let output = Colour::from_hex("#ffff00").unwrap();
        let expected = Colour {
            red: 255,
            green: 255,
            blue: 0,
        };
        assert_eq!(output, expected);
    }

    #[test]
    fn colour_from_hex_upper() {
        let output = Colour::from_hex("#ABCDEF").unwrap();
        let expected = Colour {
            red: 171,
            green: 205,
            blue: 239,
        };
        assert_eq!(output, expected);
    }
}
