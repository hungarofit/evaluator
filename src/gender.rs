use std::convert::TryFrom;
use std::fmt;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub const fn is_male(&self) -> bool {
        matches!(self, Gender::Male)
    }
}

impl TryFrom<&str> for Gender {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "m" | "male" => Ok(Self::Male),
            "f" | "female" => Ok(Self::Female),
            _ => Err(format!(
                "Invalid gender '{}'. Use 'm'/'male' or 'f'/'female'",
                s
            )),
        }
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Male => write!(f, "Male"),
            Self::Female => write!(f, "Female"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_try_from_str() {
        assert_eq!(Gender::try_from("m").unwrap(), Gender::Male);
        assert_eq!(Gender::try_from("M").unwrap(), Gender::Male);
        assert_eq!(Gender::try_from("male").unwrap(), Gender::Male);
        assert_eq!(Gender::try_from("Male").unwrap(), Gender::Male);
        assert_eq!(Gender::try_from("MALE").unwrap(), Gender::Male);

        assert_eq!(Gender::try_from("f").unwrap(), Gender::Female);
        assert_eq!(Gender::try_from("F").unwrap(), Gender::Female);
        assert_eq!(Gender::try_from("female").unwrap(), Gender::Female);
        assert_eq!(Gender::try_from("Female").unwrap(), Gender::Female);
        assert_eq!(Gender::try_from("FEMALE").unwrap(), Gender::Female);

        assert!(Gender::try_from("x").is_err());
        assert!(Gender::try_from("").is_err());
    }

    #[test]
    fn test_is_male() {
        assert!(Gender::Male.is_male());
        assert!(!Gender::Female.is_male());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Gender::Male), "Male");
        assert_eq!(format!("{}", Gender::Female), "Female");
    }
}
