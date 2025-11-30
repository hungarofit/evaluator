use thiserror::Error;

use crate::challenge::{Challenge, ChallengeResult, Classification, ChallengeScore};
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
        challenge: Challenge,
        gender: Gender,
        age: u8,
        aerob_result: f32,
        motor_results: Vec<f32>,
    ) -> Result<ChallengeResult, EvaluatorError> {
        let motor_exercises = challenge.challenge_type.motor_exercises();
        if motor_exercises.len() != motor_results.len() {
            return Err(EvaluatorError::InvalidChallenge(format!(
                "Expected {} motor results, but got {}",
                motor_exercises.len(),
                motor_results.len()
            )));
        }

        let mut motor_scores = Vec::with_capacity(motor_results.len());
        for (i, &exercise) in motor_exercises.iter().enumerate() {
            let result = motor_results[i];
            let score = self.evaluate(exercise, gender, age, result, Some(challenge.challenge_type))?;
            motor_scores.push(ChallengeScore { exercise, score });
        }
        
        let aerob_score = self.evaluate(challenge.aerob_exercise, gender, age, aerob_result, Some(challenge.challenge_type))?;
        let aerob_score = ChallengeScore { exercise: challenge.aerob_exercise, score: aerob_score};

        Ok(ChallengeResult::new(motor_scores, aerob_score))
    }


    /// Get the classification for a given total score
    pub fn classify_score(&self, total_score: f32) -> Classification {
        Classification::from_score(total_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{challenge::Challenge, tables::load_tables};

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
    fn test_evaluate_challenge_hungarofit() {
        let tables = get_test_tables();
        let evaluator = Evaluator::new(&tables);
        let challenge = Challenge::new(ChallengeType::Hungarofit, Exercise::AerobRun2Km);

        let result = evaluator.evaluate_challenge(
            challenge,
            Gender::Male,
            12,
            480.0,
            vec![2.0, 40.0, 50.0, 8.0, 14.0, 10.0],
        );
        
        assert!(result.is_ok());
        let challenge_result = result.unwrap();
        assert_eq!(challenge_result.motor_scores.len(), 6);
        assert!(challenge_result.total_score > 0.0);
    }

    #[test]
    fn test_evaluate_challenge_hungarofit_mini() {
        let tables = get_test_tables();
        let evaluator = Evaluator::new(&tables);
        let challenge = Challenge::new(ChallengeType::HungarofitMini, Exercise::AerobRun2Km);

        let result = evaluator.evaluate_challenge(
            challenge,
            Gender::Male,
            12,
            480.0,
            vec![2.0, 40.0, 50.0, 10.0],
        );
        
        assert!(result.is_ok());
        let challenge_result = result.unwrap();
        assert_eq!(challenge_result.motor_scores.len(), 4);
        assert!(challenge_result.total_score > 0.0);
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
