use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Exercise {
    // Motor exercises (shared between Motor4 and Motor6)
    Jump,
    Pushup,
    Situp,
    Torso,
    // Motor6-only exercises
    ThrowDouble,
    ThrowSingle,
    // Aerob exercises (fully qualified)
    AerobBike12Min,
    AerobRun12Min,
    AerobRun1Mile,
    AerobRun1Mile5,
    AerobRun2Km,
    AerobRun2Mile,
    AerobRun3Km,
    AerobRun6Min,
    AerobSwim12Min,
    AerobSwim500M,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChallengeType {
    Hungarofit,
    HungarofitMini,
}

impl Exercise {
    /// Resolve to table name based on challenge context
    /// For motor exercises, requires challenge context to determine motor4 vs motor6
    pub fn table_name(&self, challenge_context: Option<ChallengeType>) -> &'static str {
        match self {
            Self::Jump => match challenge_context {
                Some(ChallengeType::Hungarofit) => "motor6-jump",
                Some(ChallengeType::HungarofitMini) => "motor4-jump",
                None => "motor6-jump", // default to motor6 if no context
            },
            Self::Pushup => match challenge_context {
                Some(ChallengeType::Hungarofit) => "motor6-pushup",
                Some(ChallengeType::HungarofitMini) => "motor4-pushup",
                None => "motor6-pushup",
            },
            Self::Situp => match challenge_context {
                Some(ChallengeType::Hungarofit) => "motor6-situp",
                Some(ChallengeType::HungarofitMini) => "motor4-situp",
                None => "motor6-situp",
            },
            Self::Torso => match challenge_context {
                Some(ChallengeType::Hungarofit) => "motor6-torso",
                Some(ChallengeType::HungarofitMini) => "motor4-torso",
                None => "motor6-torso",
            },
            Self::ThrowDouble => "motor6-throwdouble",
            Self::ThrowSingle => "motor6-throwsingle",
            Self::AerobBike12Min => "aerob-bike-12min",
            Self::AerobRun12Min => "aerob-run-12min",
            Self::AerobRun1Mile => "aerob-run-1mile",
            Self::AerobRun1Mile5 => "aerob-run-1mile5",
            Self::AerobRun2Km => "aerob-run-2km",
            Self::AerobRun2Mile => "aerob-run-2mile",
            Self::AerobRun3Km => "aerob-run-3km",
            Self::AerobRun6Min => "aerob-run-6min",
            Self::AerobSwim12Min => "aerob-swim-12min",
            Self::AerobSwim500M => "aerob-swim-500m",
        }
    }

    pub fn is_aerob(&self) -> bool {
        matches!(
            self,
            Self::AerobBike12Min
                | Self::AerobRun12Min
                | Self::AerobRun1Mile
                | Self::AerobRun1Mile5
                | Self::AerobRun2Km
                | Self::AerobRun2Mile
                | Self::AerobRun3Km
                | Self::AerobRun6Min
                | Self::AerobSwim12Min
                | Self::AerobSwim500M
        )
    }

    pub fn is_motor(&self) -> bool {
        matches!(
            self,
            Self::Jump
                | Self::Pushup
                | Self::Situp
                | Self::Torso
                | Self::ThrowDouble
                | Self::ThrowSingle
        )
    }

    pub fn is_motor6_only(&self) -> bool {
        matches!(self, Self::ThrowDouble | Self::ThrowSingle)
    }

    /// Get all motor exercises
    pub const fn all_motor() -> &'static [Exercise] {
        &[
            Self::Jump,
            Self::Pushup,
            Self::Situp,
            Self::Torso,
            Self::ThrowDouble,
            Self::ThrowSingle,
        ]
    }

    /// Get all aerob exercises
    pub const fn all_aerob() -> &'static [Exercise] {
        &[
            Self::AerobBike12Min,
            Self::AerobRun12Min,
            Self::AerobRun1Mile,
            Self::AerobRun1Mile5,
            Self::AerobRun2Km,
            Self::AerobRun2Mile,
            Self::AerobRun3Km,
            Self::AerobRun6Min,
            Self::AerobSwim12Min,
            Self::AerobSwim500M,
        ]
    }

    /// Get all exercises in stable order for table generation
    /// This order must remain stable to maintain binary compatibility
    pub const fn all_exercises_ordered() -> &'static [(Exercise, bool)] {
        &[
            // Aerob exercises
            (Self::AerobBike12Min, false),
            (Self::AerobRun12Min, false),
            (Self::AerobRun1Mile, true),
            (Self::AerobRun1Mile5, true),
            (Self::AerobRun2Km, true),
            (Self::AerobRun2Mile, true),
            (Self::AerobRun3Km, true),
            (Self::AerobRun6Min, false),
            (Self::AerobSwim12Min, false),
            (Self::AerobSwim500M, true),
            // Motor4 exercises
            (Self::Jump, false),      // motor4-jump
            (Self::Pushup, false),    // motor4-pushup
            (Self::Situp, false),     // motor4-situp
            (Self::Torso, false),     // motor4-torso
            // Motor6 exercises
            (Self::Jump, false),      // motor6-jump
            (Self::Pushup, false),    // motor6-pushup
            (Self::Situp, false),     // motor6-situp
            (Self::ThrowDouble, false),
            (Self::ThrowSingle, false),
            (Self::Torso, false),     // motor6-torso
        ]
    }

    /// Get the table index for this exercise and gender
    /// Returns the index into the sheets array in AllTables
    pub fn table_index(&self, male: bool, is_motor6: bool) -> usize {
        let base_idx = match self {
            // Aerob exercises (indices 0-9, 2 sheets each = 0-19)
            Self::AerobBike12Min => 0,
            Self::AerobRun12Min => 1,
            Self::AerobRun1Mile => 2,
            Self::AerobRun1Mile5 => 3,
            Self::AerobRun2Km => 4,
            Self::AerobRun2Mile => 5,
            Self::AerobRun3Km => 6,
            Self::AerobRun6Min => 7,
            Self::AerobSwim12Min => 8,
            Self::AerobSwim500M => 9,
            // Motor exercises (indices 10-19, 2 sheets each)
            Self::Jump => if is_motor6 { 14 } else { 10 },
            Self::Pushup => if is_motor6 { 15 } else { 11 },
            Self::Situp => if is_motor6 { 16 } else { 12 },
            Self::Torso => if is_motor6 { 19 } else { 13 },
            Self::ThrowDouble => 17,
            Self::ThrowSingle => 18,
        };
        base_idx * 2 + if male { 1 } else { 0 }
    }

    /// Check if this exercise uses time-based measurement (descending = lower is better)
    pub fn is_descending(&self) -> bool {
        matches!(
            self,
            Self::AerobRun1Mile
                | Self::AerobRun1Mile5
                | Self::AerobRun2Km
                | Self::AerobRun2Mile
                | Self::AerobRun3Km
                | Self::AerobSwim500M
        )
    }
}

impl ChallengeType {
    pub fn motor_exercises(&self) -> &'static [Exercise] {
        match self {
            Self::Hungarofit => &[
                Exercise::Jump,
                Exercise::Pushup,
                Exercise::Situp,
                Exercise::ThrowDouble,
                Exercise::ThrowSingle,
                Exercise::Torso,
            ],
            Self::HungarofitMini => &[
                Exercise::Jump,
                Exercise::Pushup,
                Exercise::Situp,
                Exercise::Torso,
            ],
        }
    }

    pub fn is_valid_motor_exercise(&self, exercise: Exercise) -> bool {
        self.motor_exercises().contains(&exercise)
    }
}

impl FromStr for Exercise {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jump" => Ok(Self::Jump),
            "pushup" => Ok(Self::Pushup),
            "situp" => Ok(Self::Situp),
            "torso" => Ok(Self::Torso),
            "throwdouble" | "throw-double" => Ok(Self::ThrowDouble),
            "throwsingle" | "throw-single" => Ok(Self::ThrowSingle),
            "aerob-bike-12min" => Ok(Self::AerobBike12Min),
            "aerob-run-12min" => Ok(Self::AerobRun12Min),
            "aerob-run-1mile" => Ok(Self::AerobRun1Mile),
            "aerob-run-1mile5" => Ok(Self::AerobRun1Mile5),
            "aerob-run-2km" => Ok(Self::AerobRun2Km),
            "aerob-run-2mile" => Ok(Self::AerobRun2Mile),
            "aerob-run-3km" => Ok(Self::AerobRun3Km),
            "aerob-run-6min" => Ok(Self::AerobRun6Min),
            "aerob-swim-12min" => Ok(Self::AerobSwim12Min),
            "aerob-swim-500m" => Ok(Self::AerobSwim500M),
            // Legacy table names
            "motor4-jump" => Ok(Self::Jump),
            "motor4-pushup" => Ok(Self::Pushup),
            "motor4-situp" => Ok(Self::Situp),
            "motor4-torso" => Ok(Self::Torso),
            "motor6-jump" => Ok(Self::Jump),
            "motor6-pushup" => Ok(Self::Pushup),
            "motor6-situp" => Ok(Self::Situp),
            "motor6-torso" => Ok(Self::Torso),
            "motor6-throwdouble" => Ok(Self::ThrowDouble),
            "motor6-throwsingle" => Ok(Self::ThrowSingle),
            _ => Err(format!("Unknown exercise: {}", s)),
        }
    }
}

impl fmt::Display for Exercise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Jump => "Jump",
            Self::Pushup => "Pushup",
            Self::Situp => "Situp",
            Self::Torso => "Torso",
            Self::ThrowDouble => "Throw Double",
            Self::ThrowSingle => "Throw Single",
            Self::AerobBike12Min => "Aerob Bike 12min",
            Self::AerobRun12Min => "Aerob Run 12min",
            Self::AerobRun1Mile => "Aerob Run 1 Mile",
            Self::AerobRun1Mile5 => "Aerob Run 1.5 Mile",
            Self::AerobRun2Km => "Aerob Run 2km",
            Self::AerobRun2Mile => "Aerob Run 2 Mile",
            Self::AerobRun3Km => "Aerob Run 3km",
            Self::AerobRun6Min => "Aerob Run 6min",
            Self::AerobSwim12Min => "Aerob Swim 12min",
            Self::AerobSwim500M => "Aerob Swim 500m",
        };
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_table_name() {
        assert_eq!(
            Exercise::Jump.table_name(Some(ChallengeType::Hungarofit)),
            "motor6-jump"
        );
        assert_eq!(
            Exercise::Jump.table_name(Some(ChallengeType::HungarofitMini)),
            "motor4-jump"
        );
        assert_eq!(Exercise::ThrowDouble.table_name(None), "motor6-throwdouble");
        assert_eq!(Exercise::AerobRun2Km.table_name(None), "aerob-run-2km");
    }

    #[test]
    fn test_exercise_is_aerob() {
        assert!(Exercise::AerobRun2Km.is_aerob());
        assert!(Exercise::AerobSwim500M.is_aerob());
        assert!(!Exercise::Jump.is_aerob());
        assert!(!Exercise::ThrowDouble.is_aerob());
    }

    #[test]
    fn test_exercise_is_motor() {
        assert!(Exercise::Jump.is_motor());
        assert!(Exercise::ThrowDouble.is_motor());
        assert!(!Exercise::AerobRun2Km.is_motor());
    }

    #[test]
    fn test_challenge_type_motor_exercises() {
        let motor6 = ChallengeType::Hungarofit.motor_exercises();
        assert_eq!(motor6.len(), 6);
        assert!(motor6.contains(&Exercise::ThrowDouble));

        let motor4 = ChallengeType::HungarofitMini.motor_exercises();
        assert_eq!(motor4.len(), 4);
        assert!(!motor4.contains(&Exercise::ThrowDouble));
    }

    #[test]
    fn test_challenge_type_validation() {
        assert!(ChallengeType::Hungarofit.is_valid_motor_exercise(Exercise::ThrowDouble));
        assert!(!ChallengeType::HungarofitMini.is_valid_motor_exercise(Exercise::ThrowDouble));
        assert!(ChallengeType::HungarofitMini.is_valid_motor_exercise(Exercise::Jump));
    }

    #[test]
    fn test_from_str() {
        assert_eq!("jump".parse::<Exercise>().unwrap(), Exercise::Jump);
        assert_eq!("motor6-jump".parse::<Exercise>().unwrap(), Exercise::Jump);
        assert_eq!(
            "aerob-run-2km".parse::<Exercise>().unwrap(),
            Exercise::AerobRun2Km
        );
        assert!("invalid".parse::<Exercise>().is_err());
    }
}
