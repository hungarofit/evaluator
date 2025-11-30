use std::fmt;
use thiserror::Error;

use crate::exercise::{ChallengeType, Exercise};
use crate::gender::Gender;
use crate::tables::AllTables;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(target_arch = "wasm32"), non_exhaustive)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub enum Classification {
    Concerning,
    Weak,
    Mediocre,
    Average,
    Good,
    Excellent,
    Outstanding,
}

impl Classification {
    pub const fn from_score(score: f32) -> Self {
        match score {
            s if s < 20.5 => Self::Concerning,
            s if s < 40.5 => Self::Weak,
            s if s < 60.5 => Self::Mediocre,
            s if s < 80.5 => Self::Average,
            s if s < 100.5 => Self::Good,
            s if s < 120.5 => Self::Excellent,
            _ => Self::Outstanding,
        }
    }

    pub fn score_range(&self) -> (f32, f32) {
        match self {
            Self::Concerning => (0.0, 20.499),
            Self::Weak => (20.5, 40.499),
            Self::Mediocre => (40.5, 60.499),
            Self::Average => (60.5, 80.499),
            Self::Good => (80.5, 100.499),
            Self::Excellent => (100.5, 120.499),
            Self::Outstanding => (120.5, 140.0),
        }
    }
}

impl fmt::Display for Classification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Concerning => "Concerning",
            Self::Weak => "Weak",
            Self::Mediocre => "Mediocre",
            Self::Average => "Average",
            Self::Good => "Good",
            Self::Excellent => "Excellent",
            Self::Outstanding => "Outstanding",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum EvaluatorError {
    #[error("Exercise '{exercise}' not found in tables")]
    ExerciseNotFound {
        exercise: Exercise,
    },
    #[error("Age {age} is out of range for exercise '{exercise}'")]
    AgeOutOfRange {
        age: u8,
        exercise: Exercise,
    },
    #[error("Challenge type is required for motor exercise '{exercise}'")]
    ChallengeTypeRequired {
        exercise: Exercise,
    },
}

pub struct Evaluator<'a> {
    tables: &'a AllTables,
}

impl<'a> Evaluator<'a> {
    pub const fn new(tables: &'a AllTables) -> Self {
        Self { tables }
    }

    #[must_use]
    pub fn evaluate(
        &self,
        exercise: Exercise,
        gender: Gender,
        age: u8,
        result: f32,
        challenge_context: Option<ChallengeType>,
    ) -> Result<f32, EvaluatorError> {
        if exercise.is_motor() && challenge_context.is_none() {
            return Err(EvaluatorError::ChallengeTypeRequired { exercise });
        }

        let table_name = exercise.table_name(challenge_context);
        let sheet = self
            .tables
            .get_sheet(table_name, gender.is_male())
            .ok_or(EvaluatorError::ExerciseNotFound { exercise })?;

        sheet
            .lookup(age, result)
            .ok_or(EvaluatorError::AgeOutOfRange { age, exercise })
    }

    pub fn classify_score(&self, total_score: f32) -> Classification {
        Classification::from_score(total_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tables::load_tables;

    fn get_test_tables() -> AllTables {
        let data = include_bytes!("../generated_tables.bin");
        load_tables(data)
    }

    #[test]
    fn test_evaluate_single_exercise() {
        let tables = get_test_tables();
        let evaluator = Evaluator::new(&tables);

        // Test a motor6 exercise with context
        let score = evaluator
            .evaluate(
                Exercise::Jump,
                Gender::Male,
                12,
                2.0,
                Some(ChallengeType::Hungarofit),
            )
            .unwrap();
        assert!(score >= 0.0);

        // Test an aerob exercise without context
        let score = evaluator
            .evaluate(Exercise::AerobRun2Km, Gender::Male, 12, 480.0, None)
            .unwrap();
        assert!(score >= 0.0);
    }
    
    #[test]
    fn test_evaluate_challenge_type_required_error() {
        let tables = get_test_tables();
        let evaluator = Evaluator::new(&tables);

        let err = evaluator
            .evaluate(
                Exercise::Jump,
                Gender::Male,
                12,
                2.0,
                None, // Missing challenge_context
            )
            .unwrap_err();

        assert!(matches!(err, EvaluatorError::ChallengeTypeRequired { exercise: Exercise::Jump }));
    }

    #[test]
    fn test_classify_score() {
        let tables = get_test_tables();
        let evaluator = Evaluator::new(&tables);

        assert_eq!(
            evaluator.classify_score(10.0),
            Classification::Concerning
        );
        assert_eq!(evaluator.classify_score(20.5), Classification::Weak);
        assert_eq!(evaluator.classify_score(40.5), Classification::Mediocre);
        assert_eq!(evaluator.classify_score(60.5), Classification::Average);
        assert_eq!(evaluator.classify_score(80.5), Classification::Good);
        assert_eq!(evaluator.classify_score(100.5), Classification::Excellent);
        assert_eq!(evaluator.classify_score(120.5), Classification::Outstanding);
    }

    #[test]
    fn test_classification_from_score() {
        assert_eq!(Classification::from_score(0.0), Classification::Concerning);
        assert_eq!(Classification::from_score(20.49), Classification::Concerning);
        assert_eq!(Classification::from_score(20.5), Classification::Weak);
        assert_eq!(Classification::from_score(40.49), Classification::Weak);
        assert_eq!(Classification::from_score(40.5), Classification::Mediocre);
        assert_eq!(Classification::from_score(60.49), Classification::Mediocre);
        assert_eq!(Classification::from_score(60.5), Classification::Average);
        assert_eq!(Classification::from_score(80.49), Classification::Average);
        assert_eq!(Classification::from_score(80.5), Classification::Good);
        assert_eq!(Classification::from_score(100.49), Classification::Good);
        assert_eq!(Classification::from_score(100.5), Classification::Excellent);
        assert_eq!(Classification::from_score(120.49), Classification::Excellent);
        assert_eq!(Classification::from_score(120.5), Classification::Outstanding);
        assert_eq!(Classification::from_score(140.0), Classification::Outstanding);
    }

    #[test]
    fn test_classification_ordering() {
        assert!(Classification::Concerning < Classification::Weak);
        assert!(Classification::Weak < Classification::Mediocre);
        assert!(Classification::Mediocre < Classification::Average);
        assert!(Classification::Good < Classification::Excellent);
        assert!(Classification::Excellent < Classification::Outstanding);
    }
}
