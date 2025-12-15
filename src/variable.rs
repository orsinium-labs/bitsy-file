#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variable {
    pub id: String,
    pub initial_value: String,
}

impl From<String> for Variable {
    fn from(string: String) -> Variable {
        let id_value: Vec<&str> = string.lines().collect();
        let id = id_value[0].replace("VAR ", "");

        let initial_value = if id_value.len() == 1 {
            "".to_string()
        } else {
            id_value[1..].join("")
        };

        Variable { id, initial_value }
    }
}

impl ToString for Variable {
    fn to_string(&self) -> String {
        format!("VAR {}\n{}", self.id, self.initial_value)
    }
}

#[cfg(test)]
mod test {
    use crate::Variable;

    #[test]
    fn variable_from_string() {
        assert_eq!(
            Variable::from("VAR a\n42".to_string()),
            Variable {
                id: "a".to_string(),
                initial_value: "42".to_string()
            }
        );
    }

    #[test]
    fn variable_to_string() {
        let output = Variable {
            id: "c".to_string(),
            initial_value: "57".to_string(),
        }
        .to_string();
        let expected = "VAR c\n57".to_string();
        assert_eq!(output, expected);
    }
}
