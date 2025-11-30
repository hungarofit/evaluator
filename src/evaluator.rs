use thiserror::Error;

use crate::challenge::{ChallengeResult, Classification};
use crate::exercise::{ChallengeType, Exercise};
use crate::gender::Gender;
use crate::tables::AllTables;

#[derive(Debug, Error)]
pub enum EvaluatorError {
    #[error("Exercise {0} not found in tables")]
    ExerciseNotFound(Exercise),

    #[error("Age {0} is out of range for this exercise")]
    AgeOutOfRange(u8),

    #[error("Invalid challenge configuration: {0}")]
    InvalidChallenge(String),

    #[error("Expected exactly 1 aerob exercise in challenge, found {0}")]
    InvalidAerobCount(usize),

    #[error("Exercise {exercise:?} is not valid for challenge {challenge:?}")]
    InvalidExerciseForChallenge {
        exercise: Exercise,
        challenge: ChallengeType,
    },

    #[error("Missing required motor exercise {0:?} for this challenge")]
    MissingMotorExercise(Exercise),
}

pub struct Evaluator<'a> {
    tables: &'a AllTables,
}

impl<'a> Evaluator<'a> {
    pub fn new(tables: &'a AllTables) -> Self {
        Self { tables }
    }

    /// Evaluate a single exercise for a given age and result
    ///
    /// # Arguments
    /// * `exercise` - The exercise to evaluate
    /// * `gender` - Gender of the person
    /// * `age` - Age of the person
    /// * `result` - The measured result (e.g., distance, time, repetitions)
    /// * `challenge_context` - Optional challenge context for motor exercises (Jump, Pushup, etc.)
    ///
    /// # Returns
    /// The score for this exercise, or an error if evaluation failed
    pub fn evaluate(
        &self,
        exercise: Exercise,
        gender: Gender,
        age: u8,
        result: f32,
        challenge_context: Option<ChallengeType>,
    ) -> Result<f32, EvaluatorError> {
        let table_name = exercise.table_name(challenge_context);
        let sheet = self
            .tables
            .get_sheet(table_name, gender.is_male())
            .ok_or(EvaluatorError::ExerciseNotFound(exercise))?;

        sheet
            .lookup(age, result)
            .ok_or(EvaluatorError::AgeOutOfRange(age))
    }

    /// Evaluate a complete challenge with all required exercises
    ///
    /// # Arguments
    /// * `challenge_type` - The type of challenge (Hungarofit or HungarofitMini)
    /// * `gender` - Gender of the person
    /// * `age` - Age of the person
    /// * `results` - Slice of (Exercise, result) tuples
    ///
    /// # Returns
    /// A `ChallengeResult` containing the total score, classification, and individual scores
    ///
    /// # Errors
    /// Returns an error if:
    /// - Not exactly 1 aerob exercise is provided
    /// - An invalid motor exercise is used for the challenge type
    /// - A required motor exercise is missing
    /// - Any exercise evaluation fails
    pub fn evaluate_challenge(
        &self,
        challenge_type: ChallengeType,
        gender: Gender,
        age: u8,
        results: &[(Exercise, f32)],
    ) -> Result<ChallengeResult, EvaluatorError> {
        // Separate motor and aerob exercises
        let (motor_results, aerob_results): (Vec<_>, Vec<_>) =
            results.iter().partition(|(ex, _)| ex.is_motor());

        // Validate exactly 1 aerob exercise
        if aerob_results.len() != 1 {
            return Err(EvaluatorError::InvalidAerobCount(aerob_results.len()));
        }

        // Validate motor exercises match challenge type
        let required_exercises = challenge_type.motor_exercises();
        for &(exercise, _) in &motor_results {
            if !challenge_type.is_valid_motor_exercise(exercise) {
                return Err(EvaluatorError::InvalidExerciseForChallenge {
                    exercise,
                    challenge: challenge_type,
                });
            }
        }

        // Check all required motor exercises are present
        for &required_exercise in required_exercises {
            if !motor_results
                .iter()
                .any(|(ex, _)| *ex == required_exercise)
            {
                return Err(EvaluatorError::MissingMotorExercise(required_exercise));
            }
        }

        // Evaluate all exercises with challenge context
        let mut exercise_scores = Vec::with_capacity(results.len());

        for &(exercise, result) in results {
            let score = self.evaluate(exercise, gender, age, result, Some(challenge_type))?;
            exercise_scores.push((exercise, score));
        }

        Ok(ChallengeResult::new(exercise_scores))
    }

    /// Evaluate a partial challenge (some exercises may be missing)
    ///
    /// This is useful for tracking progress or when not all exercises have been completed yet.
    /// Note that the classification may not be meaningful for partial results.
    pub fn evaluate_partial_challenge(
        &self,
        challenge_type: ChallengeType,
        gender: Gender,
        age: u8,
        results: &[(Exercise, f32)],
    ) -> Result<ChallengeResult, EvaluatorError> {
        // Separate motor and aerob exercises
        let (motor_results, aerob_results): (Vec<_>, Vec<_>) =
            results.iter().partition(|(ex, _)| ex.is_motor());

        // Validate at most 1 aerob exercise
        if aerob_results.len() > 1 {
            return Err(EvaluatorError::InvalidAerobCount(aerob_results.len()));
        }

        // Validate motor exercises match challenge type
        for &(exercise, _) in &motor_results {
            if !challenge_type.is_valid_motor_exercise(exercise) {
                return Err(EvaluatorError::InvalidExerciseForChallenge {
                    exercise,
                    challenge: challenge_type,
                });
            }
        }

        // Evaluate all exercises with challenge context
        let mut exercise_scores = Vec::with_capacity(results.len());

        for &(exercise, result) in results {
            let score = self.evaluate(exercise, gender, age, result, Some(challenge_type))?;
            exercise_scores.push((exercise, score));
        }

        Ok(ChallengeResult::new(exercise_scores))
    }

    /// Get the classification for a given total score
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
    fn test_evaluate_challenge_invalid_aerob_count() {
        let tables = get_test_tables();
        let evaluator = Evaluator::new(&tables);

        // No aerob exercise
        let result = evaluator.evaluate_challenge(
            ChallengeType::Hungarofit,
            Gender::Male,
            12,
            &[
                (Exercise::Jump, 2.0),
                (Exercise::Pushup, 40.0),
                (Exercise::Situp, 50.0),
                (Exercise::ThrowDouble, 8.0),
                (Exercise::ThrowSingle, 14.0),
                (Exercise::Torso, 10.0),
            ],
        );
        assert!(matches!(
            result,
            Err(EvaluatorError::InvalidAerobCount(0))
        ));

        // Two aerob exercises
        let result = evaluator.evaluate_challenge(
            ChallengeType::Hungarofit,
            Gender::Male,
            12,
            &[
                (Exercise::Jump, 2.0),
                (Exercise::Pushup, 40.0),
                (Exercise::Situp, 50.0),
                (Exercise::ThrowDouble, 8.0),
                (Exercise::ThrowSingle, 14.0),
                (Exercise::Torso, 10.0),
                (Exercise::AerobRun2Km, 480.0),
                (Exercise::AerobRun3Km, 720.0),
            ],
        );
        assert!(matches!(
            result,
            Err(EvaluatorError::InvalidAerobCount(2))
        ));
    }

    #[test]
    fn test_evaluate_challenge_invalid_motor_exercise() {
        let tables = get_test_tables();
        let evaluator = Evaluator::new(&tables);

        // ThrowDouble not valid for HungarofitMini
        let result = evaluator.evaluate_challenge(
            ChallengeType::HungarofitMini,
            Gender::Male,
            12,
            &[
                (Exercise::Jump, 2.0),
                (Exercise::Pushup, 40.0),
                (Exercise::Situp, 50.0),
                (Exercise::Torso, 10.0),
                (Exercise::ThrowDouble, 8.0), // Invalid for Motor4
                (Exercise::AerobRun2Km, 480.0),
            ],
        );
        assert!(matches!(
            result,
            Err(EvaluatorError::InvalidExerciseForChallenge { .. })
        ));
    }

    #[test]
    fn test_evaluate_challenge_missing_motor_exercise() {
        let tables = get_test_tables();
        let evaluator = Evaluator::new(&tables);

        // Missing Pushup
        let result = evaluator.evaluate_challenge(
            ChallengeType::HungarofitMini,
            Gender::Male,
            12,
            &[
                (Exercise::Jump, 2.0),
                (Exercise::Situp, 50.0),
                (Exercise::Torso, 10.0),
                (Exercise::AerobRun2Km, 480.0),
            ],
        );
        assert!(matches!(
            result,
            Err(EvaluatorError::MissingMotorExercise(_))
        ));
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
}
