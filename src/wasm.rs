use wasm_bindgen::prelude::*;
use crate::{
    Evaluator, 
    Gender as InternalGender, 
    Exercise as InternalExercise, 
    ChallengeType as InternalChallengeType, 
    load_embedded_tables, 
    AllTables, 
    Classification as InternalClassification
};
use once_cell::sync::Lazy;

// Static reference to loaded tables
static TABLES: Lazy<AllTables> = Lazy::new(|| load_embedded_tables());

// ============================================================================
// WASM-exported Enums
// ============================================================================

/// Gender enum for TypeScript
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

impl From<Gender> for InternalGender {
    fn from(g: Gender) -> Self {
        match g {
            Gender::Male => InternalGender::Male,
            Gender::Female => InternalGender::Female,
        }
    }
}

/// Exercise enum for TypeScript
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exercise {
    Jump,
    Pushup,
    Situp,
    Torso,
    ThrowDouble,
    ThrowSingle,
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

impl From<Exercise> for InternalExercise {
    fn from(e: Exercise) -> Self {
        match e {
            Exercise::Jump => InternalExercise::Jump,
            Exercise::Pushup => InternalExercise::Pushup,
            Exercise::Situp => InternalExercise::Situp,
            Exercise::Torso => InternalExercise::Torso,
            Exercise::ThrowDouble => InternalExercise::ThrowDouble,
            Exercise::ThrowSingle => InternalExercise::ThrowSingle,
            Exercise::AerobBike12Min => InternalExercise::AerobBike12Min,
            Exercise::AerobRun12Min => InternalExercise::AerobRun12Min,
            Exercise::AerobRun1Mile => InternalExercise::AerobRun1Mile,
            Exercise::AerobRun1Mile5 => InternalExercise::AerobRun1Mile5,
            Exercise::AerobRun2Km => InternalExercise::AerobRun2Km,
            Exercise::AerobRun2Mile => InternalExercise::AerobRun2Mile,
            Exercise::AerobRun3Km => InternalExercise::AerobRun3Km,
            Exercise::AerobRun6Min => InternalExercise::AerobRun6Min,
            Exercise::AerobSwim12Min => InternalExercise::AerobSwim12Min,
            Exercise::AerobSwim500M => InternalExercise::AerobSwim500M,
        }
    }
}

/// Challenge type enum for TypeScript
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChallengeType {
    Hungarofit,
    HungarofitMini,
}

impl From<ChallengeType> for InternalChallengeType {
    fn from(c: ChallengeType) -> Self {
        match c {
            ChallengeType::Hungarofit => InternalChallengeType::Hungarofit,
            ChallengeType::HungarofitMini => InternalChallengeType::HungarofitMini,
        }
    }
}

/// Classification enum for TypeScript
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Classification {
    Concerning,
    Weak,
    Mediocre,
    Average,
    Good,
    Excellent,
    Outstanding,
}

impl From<InternalClassification> for Classification {
    fn from(c: InternalClassification) -> Self {
        match c {
            InternalClassification::Concerning => Classification::Concerning,
            InternalClassification::Weak => Classification::Weak,
            InternalClassification::Mediocre => Classification::Mediocre,
            InternalClassification::Average => Classification::Average,
            InternalClassification::Good => Classification::Good,
            InternalClassification::Excellent => Classification::Excellent,
            InternalClassification::Outstanding => Classification::Outstanding,
        }
    }
}

/// Initialize the evaluator with embedded tables
/// This must be called before using any other functions
#[wasm_bindgen]
pub fn init() -> WasmEvaluator {
    WasmEvaluator::new()
}

/// WASM wrapper for the Evaluator
#[wasm_bindgen]
pub struct WasmEvaluator;

#[wasm_bindgen]
impl WasmEvaluator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self
    }

    /// Evaluate a single exercise result using enum types
    /// 
    /// # Arguments
    /// * `exercise` - Exercise enum
    /// * `age` - Age in years
    /// * `gender` - Gender enum
    /// * `result` - The result value (distance, time, count, etc.)
    /// * `challenge_type` - Optional challenge type enum
    /// 
    /// # Returns
    /// The score (0.0 to 5.0) or null if the age is not found
    pub fn evaluate_typed(
        &self, 
        exercise: Exercise, 
        age: u8, 
        gender: Gender, 
        result: f32, 
        challenge_type: Option<ChallengeType>
    ) -> Option<f32> {
        let evaluator = Evaluator::new(&TABLES);
        let exercise_enum: InternalExercise = exercise.into();
        let gender_enum: InternalGender = gender.into();
        let challenge = challenge_type.map(|c| c.into());
        
        evaluator.evaluate(exercise_enum, gender_enum, age, result, challenge).ok()
    }

    /// Evaluate a single exercise result using string types (legacy)
    /// 
    /// # Arguments
    /// * `exercise` - Exercise name (e.g., "aerob-run-12min", "motor6-jump")
    /// * `age` - Age in years
    /// * `is_male` - true for male, false for female
    /// * `result` - The result value (distance, time, count, etc.)
    /// * `challenge_type` - Optional challenge type: "hungarofit", "hungarofit-mini", or null
    /// 
    /// # Returns
    /// The score (0.0 to 5.0) or null if the exercise or age is not found
    pub fn evaluate(&self, exercise: &str, age: u8, is_male: bool, result: f32, challenge_type: Option<String>) -> Option<f32> {
        let evaluator = Evaluator::new(&TABLES);
        let gender = if is_male { InternalGender::Male } else { InternalGender::Female };
        
        // Parse exercise from string
        let exercise_enum = InternalExercise::from_name(exercise)?;
        
        // Parse challenge type if provided
        let challenge = challenge_type.and_then(|ct| match ct.to_lowercase().as_str() {
            "hungarofit" => Some(InternalChallengeType::Hungarofit),
            "hungarofit-mini" | "hungarofitmini" => Some(InternalChallengeType::HungarofitMini),
            _ => None,
        });
        
        evaluator.evaluate(exercise_enum, gender, age, result, challenge).ok()
    }
    
    /// Get the classification for a score (returns enum)
    /// 
    /// # Arguments
    /// * `score` - The total score
    /// 
    /// # Returns
    /// Classification enum
    pub fn get_classification(&self, score: f32) -> Classification {
        let classification = InternalClassification::from_score(score);
        classification.into()
    }

    /// Get the classification for a score as string (legacy)
    /// 
    /// # Arguments
    /// * `score` - The total score
    /// 
    /// # Returns
    /// Classification string: "excellent", "good", "average", "mediocre", "weak", "concerning", or "outstanding"
    pub fn get_classification_string(&self, score: f32) -> String {
        let classification = InternalClassification::from_score(score);
        classification_to_string(classification)
    }

    /// Get all available exercise names
    /// 
    /// # Returns
    /// Array of exercise names that can be used in evaluate()
    pub fn get_all_exercises(&self) -> Vec<String> {
        vec![
            "jump".to_string(),
            "pushup".to_string(),
            "situp".to_string(),
            "torso".to_string(),
            "throw-double".to_string(),
            "throw-single".to_string(),
            "aerob-run-12min".to_string(),
            "aerob-run-6min".to_string(),
            "aerob-run-3km".to_string(),
            "aerob-run-2km".to_string(),
            "aerob-run-2mile".to_string(),
            "aerob-run-1mile5".to_string(),
            "aerob-run-1mile".to_string(),
            "aerob-swim-12min".to_string(),
            "aerob-swim-500m".to_string(),
            "aerob-bike-12min".to_string(),
        ]
    }

    /// Get all motor exercises
    pub fn get_motor_exercises(&self) -> Vec<String> {
        vec![
            "jump".to_string(),
            "pushup".to_string(),
            "situp".to_string(),
            "torso".to_string(),
            "throw-double".to_string(),
            "throw-single".to_string(),
        ]
    }

    /// Get all aerob exercises
    pub fn get_aerob_exercises(&self) -> Vec<String> {
        vec![
            "aerob-run-12min".to_string(),
            "aerob-run-6min".to_string(),
            "aerob-run-3km".to_string(),
            "aerob-run-2km".to_string(),
            "aerob-run-2mile".to_string(),
            "aerob-run-1mile5".to_string(),
            "aerob-run-1mile".to_string(),
            "aerob-swim-12min".to_string(),
            "aerob-swim-500m".to_string(),
            "aerob-bike-12min".to_string(),
        ]
    }

    /// Get motor exercises for a specific challenge type
    /// 
    /// # Arguments
    /// * `challenge_type` - "hungarofit" or "hungarofit-mini"
    /// 
    /// # Returns
    /// Array of exercise names valid for this challenge, or null if invalid challenge type
    pub fn get_motor_exercises_for_challenge(&self, challenge_type: &str) -> Option<Vec<String>> {
        match challenge_type.to_lowercase().as_str() {
            "hungarofit" => Some(vec![
                "jump".to_string(),
                "pushup".to_string(),
                "situp".to_string(),
                "throw-double".to_string(),
                "throw-single".to_string(),
                "torso".to_string(),
            ]),
            "hungarofit-mini" | "hungarofitmini" => Some(vec![
                "jump".to_string(),
                "pushup".to_string(),
                "situp".to_string(),
                "torso".to_string(),
            ]),
            _ => None,
        }
    }

    /// Get display name for an exercise
    /// 
    /// # Arguments
    /// * `exercise` - Exercise name
    /// 
    /// # Returns
    /// Human-readable display name or null if exercise not found
    pub fn get_exercise_display_name(&self, exercise: &str) -> Option<String> {
        let ex = InternalExercise::from_name(exercise)?;
        Some(format!("{}", ex))
    }

    /// Check if an exercise is valid for a challenge type
    /// 
    /// # Arguments
    /// * `exercise` - Exercise name
    /// * `challenge_type` - "hungarofit" or "hungarofit-mini"
    /// 
    /// # Returns
    /// true if the exercise is valid for this challenge, false otherwise
    pub fn is_exercise_valid_for_challenge(&self, exercise: &str, challenge_type: &str) -> bool {
        let ex = match InternalExercise::from_name(exercise) {
            Some(e) => e,
            None => return false,
        };

        let challenge = match challenge_type.to_lowercase().as_str() {
            "hungarofit" => InternalChallengeType::Hungarofit,
            "hungarofit-mini" | "hungarofitmini" => InternalChallengeType::HungarofitMini,
            _ => return false,
        };

        // Aerob exercises are valid for all challenges
        if ex.is_aerob() {
            return true;
        }

        // Motor exercises must be validated
        challenge.is_valid_motor_exercise(ex)
    }

    /// Get the score range for a classification
    /// 
    /// # Arguments
    /// * `classification` - One of: "concerning", "weak", "mediocre", "average", "good", "excellent", "outstanding"
    /// 
    /// # Returns
    /// Array [min, max] representing the score range, or null if invalid classification
    pub fn get_classification_range(&self, classification: &str) -> Option<Vec<f32>> {
        let class = match classification.to_lowercase().as_str() {
            "concerning" => InternalClassification::Concerning,
            "weak" => InternalClassification::Weak,
            "mediocre" => InternalClassification::Mediocre,
            "average" => InternalClassification::Average,
            "good" => InternalClassification::Good,
            "excellent" => InternalClassification::Excellent,
            "outstanding" => InternalClassification::Outstanding,
            _ => return None,
        };

        let (min, max) = class.score_range();
        Some(vec![min, max])
    }

    /// Get all classification levels in order
    /// 
    /// # Returns
    /// Array of classification strings from lowest to highest
    pub fn get_all_classifications(&self) -> Vec<String> {
        vec![
            "concerning".to_string(),
            "weak".to_string(),
            "mediocre".to_string(),
            "average".to_string(),
            "good".to_string(),
            "excellent".to_string(),
            "outstanding".to_string(),
        ]
    }

    /// Check if an exercise uses descending measurement (lower is better)
    /// 
    /// # Arguments
    /// * `exercise` - Exercise name
    /// 
    /// # Returns
    /// true if lower values are better (e.g., race times), false if higher values are better
    pub fn is_exercise_descending(&self, exercise: &str) -> bool {
        match InternalExercise::from_name(exercise) {
            Some(ex) => ex.is_descending(),
            None => false,
        }
    }

    /// Get available challenge types
    /// 
    /// # Returns
    /// Array of challenge type strings
    pub fn get_challenge_types(&self) -> Vec<String> {
        vec![
            "hungarofit".to_string(),
            "hungarofit-mini".to_string(),
        ]
    }
}

impl InternalExercise {
    /// Parse exercise from string name
    fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().replace("-", "").replace("_", "").as_str() {
            "jump" | "motor6jump" | "motor4jump" => Some(InternalExercise::Jump),
            "pushup" | "motor6pushup" | "motor4pushup" => Some(InternalExercise::Pushup),
            "situp" | "motor6situp" | "motor4situp" => Some(InternalExercise::Situp),
            "torso" | "motor6torso" | "motor4torso" => Some(InternalExercise::Torso),
            "throwdouble" | "motor6throwdouble" => Some(InternalExercise::ThrowDouble),
            "throwsingle" | "motor6throwsingle" => Some(InternalExercise::ThrowSingle),
            "aerobrun12min" => Some(InternalExercise::AerobRun12Min),
            "aerobrun6min" => Some(InternalExercise::AerobRun6Min),
            "aerobrun3km" => Some(InternalExercise::AerobRun3Km),
            "aerobrun2km" => Some(InternalExercise::AerobRun2Km),
            "aerobrun2mile" => Some(InternalExercise::AerobRun2Mile),
            "aerobrun1mile5" => Some(InternalExercise::AerobRun1Mile5),
            "aerobrun1mile" => Some(InternalExercise::AerobRun1Mile),
            "aerobswim12min" => Some(InternalExercise::AerobSwim12Min),
            "aerobswim500m" => Some(InternalExercise::AerobSwim500M),
            "aerobbike12min" => Some(InternalExercise::AerobBike12Min),
            _ => None,
        }
    }
}

/// Helper function to convert Classification to string
fn classification_to_string(classification: InternalClassification) -> String {
    match classification {
        InternalClassification::Outstanding => "outstanding".to_string(),
        InternalClassification::Excellent => "excellent".to_string(),
        InternalClassification::Good => "good".to_string(),
        InternalClassification::Average => "average".to_string(),
        InternalClassification::Mediocre => "mediocre".to_string(),
        InternalClassification::Weak => "weak".to_string(),
        InternalClassification::Concerning => "concerning".to_string(),
    }
}
