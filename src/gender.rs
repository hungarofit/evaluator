use std::fmt;
use std::str::FromStr;

/// Represents the gender/sex of a person for exercise evaluation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    /// Returns true if Male, false if Female
    /// 
    /// This is used for backward compatibility with table lookups
    /// where Hungarian datasets use "fiúk" (boys) and "lányok" (girls)
    pub fn is_male(&self) -> bool {
        matches!(self, Gender::Male)
    }
}

impl FromStr for Gender {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "m" | "male" => Ok(Gender::Male),
            "f" | "female" => Ok(Gender::Female),
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
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("m".parse::<Gender>().unwrap(), Gender::Male);
        assert_eq!("M".parse::<Gender>().unwrap(), Gender::Male);
        assert_eq!("male".parse::<Gender>().unwrap(), Gender::Male);
        assert_eq!("Male".parse::<Gender>().unwrap(), Gender::Male);
        assert_eq!("MALE".parse::<Gender>().unwrap(), Gender::Male);

        assert_eq!("f".parse::<Gender>().unwrap(), Gender::Female);
        assert_eq!("F".parse::<Gender>().unwrap(), Gender::Female);
        assert_eq!("female".parse::<Gender>().unwrap(), Gender::Female);
        assert_eq!("Female".parse::<Gender>().unwrap(), Gender::Female);
        assert_eq!("FEMALE".parse::<Gender>().unwrap(), Gender::Female);

        assert!("x".parse::<Gender>().is_err());
        assert!("".parse::<Gender>().is_err());
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
