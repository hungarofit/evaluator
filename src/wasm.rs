use wasm_bindgen::prelude::*;
use crate::{
    challenge::ChallengeScore,
    challenge::Challenge as InternalChallenge,
    Classification as InternalClassification,
    AllTables,
    load_embedded_tables,
    ChallengeType,
    Exercise as InternalExercise,
    Gender as InternalGender,
    Evaluator as InternalEvaluator
};
use once_cell::sync::Lazy;
use wasm_bindgen::JsValue;

#[wasm_bindgen(inline_js = r#"
export function createObject() { return {}; }
export function createArray() { return []; }
export function setProperty(obj, key, value) { obj[key] = value; }
export function pushArray(arr, value) { arr.push(value); }
"#)]
extern "C" {
    fn createObject() -> JsValue;
    fn createArray() -> JsValue;
    fn setProperty(obj: &JsValue, key: &str, value: &JsValue);
    fn pushArray(arr: &JsValue, value: &JsValue);
}

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

/// The result of a full evaluation
#[wasm_bindgen]
#[derive(Debug)]
pub struct Evaluation {
    #[wasm_bindgen(skip)]
    pub motor_scores: Vec<ChallengeScore>,
    #[wasm_bindgen(skip)]
    pub aerob_score: ChallengeScore,
    pub total_score: f32,
    pub classification: Classification,
}

#[wasm_bindgen]
impl Evaluation {
    #[wasm_bindgen(getter)]
    pub fn motor_scores(&self) -> Vec<f32> {
        self.motor_scores.iter().map(|s| s.score).collect()
    }

    #[wasm_bindgen(getter)]
    pub fn aerob_score(&self) -> f32 {
        self.aerob_score.score
    }
    
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> JsValue {
        let obj = createObject();
        setProperty(&obj, "total_score", &self.total_score.into());
        setProperty(&obj, "classification", &(self.classification as u32).into());
        setProperty(&obj, "aerob_score", &self.aerob_score().into());
        
        let arr = createArray();
        for score in &self.motor_scores {
            pushArray(&arr, &score.score.into());
        }
        setProperty(&obj, "motor_scores", &arr);
        
        obj
    }
}

// ============================================================================
// Internal Helper for Evaluation
// ============================================================================

fn perform_evaluation(
    challenge_type: ChallengeType,
    aerob_exercise: InternalExercise,
    gender: InternalGender,
    age: u8,
    aerob_result: f32,
    results: Vec<f32>
) -> Option<Evaluation> {
    let evaluator = InternalEvaluator::new(&TABLES);
    let challenge = InternalChallenge::new(challenge_type, aerob_exercise);
    let evaluation = evaluator.evaluate_challenge(challenge, gender, age, aerob_result, results).ok()?;
    
    Some(Evaluation {
        motor_scores: evaluation.motor_scores,
        aerob_score: evaluation.aerob_score,
        total_score: evaluation.total_score,
        classification: evaluation.classification.into(),
    })
}

// ============================================================================
// Hungarofit Evaluator (Motor6)
// ============================================================================

#[wasm_bindgen]
pub struct Evaluator {
    aerob_exercise: InternalExercise,
}

#[wasm_bindgen]
pub struct EvaluatorWithAge {
    aerob_exercise: InternalExercise,
    age: u8,
}

#[wasm_bindgen]
pub struct EvaluatorWithGender {
    aerob_exercise: InternalExercise,
    gender: InternalGender,
}

#[wasm_bindgen]
pub struct EvaluatorWithAgeGender {
    aerob_exercise: InternalExercise,
    age: u8,
    gender: InternalGender,
}

#[wasm_bindgen]
impl Evaluator {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise) -> Self {
        Self { aerob_exercise: aerob_exercise.into() }
    }

    pub fn with_age(self, age: u8) -> EvaluatorWithAge {
        EvaluatorWithAge { aerob_exercise: self.aerob_exercise, age }
    }

    pub fn with_gender(self, gender: Gender) -> EvaluatorWithGender {
        EvaluatorWithGender { aerob_exercise: self.aerob_exercise, gender: gender.into() }
    }

    pub fn evaluate(&self, gender: Gender, age: u8, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32, throw_double: f32, throw_single: f32) -> Option<Evaluation> {
        let results = vec![jump, pushup, situp, torso, throw_double, throw_single];
        perform_evaluation(ChallengeType::Hungarofit, self.aerob_exercise, gender.into(), age, aerob_result, results)
    }
}

#[wasm_bindgen]
impl EvaluatorWithAge {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise, age: u8) -> Self {
        Self { aerob_exercise: aerob_exercise.into(), age }
    }

    pub fn with_gender(self, gender: Gender) -> EvaluatorWithAgeGender {
        EvaluatorWithAgeGender { aerob_exercise: self.aerob_exercise, age: self.age, gender: gender.into() }
    }
    
    pub fn evaluate(&self, gender: Gender, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32, throw_double: f32, throw_single: f32) -> Option<Evaluation> {
        let results = vec![jump, pushup, situp, torso, throw_double, throw_single];
        perform_evaluation(ChallengeType::Hungarofit, self.aerob_exercise, gender.into(), self.age, aerob_result, results)
    }
}

#[wasm_bindgen]
impl EvaluatorWithGender {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise, gender: Gender) -> Self {
        Self { aerob_exercise: aerob_exercise.into(), gender: gender.into() }
    }

    pub fn with_age(self, age: u8) -> EvaluatorWithAgeGender {
        EvaluatorWithAgeGender { aerob_exercise: self.aerob_exercise, gender: self.gender, age }
    }

    pub fn evaluate(&self, age: u8, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32, throw_double: f32, throw_single: f32) -> Option<Evaluation> {
        let results = vec![jump, pushup, situp, torso, throw_double, throw_single];
        perform_evaluation(ChallengeType::Hungarofit, self.aerob_exercise, self.gender, age, aerob_result, results)
    }
}

#[wasm_bindgen]
impl EvaluatorWithAgeGender {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise, age: u8, gender: Gender) -> Self {
        Self { aerob_exercise: aerob_exercise.into(), age, gender: gender.into() }
    }

    pub fn evaluate(&self, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32, throw_double: f32, throw_single: f32) -> Option<Evaluation> {
        let results = vec![jump, pushup, situp, torso, throw_double, throw_single];
        perform_evaluation(ChallengeType::Hungarofit, self.aerob_exercise, self.gender, self.age, aerob_result, results)
    }
}

// ============================================================================
// HungarofitMini Evaluator (Motor4)
// ============================================================================

#[wasm_bindgen]
pub struct EvaluatorMini {
    aerob_exercise: InternalExercise,
}

#[wasm_bindgen]
pub struct EvaluatorMiniWithAge {
    aerob_exercise: InternalExercise,
    age: u8,
}

#[wasm_bindgen]
pub struct EvaluatorMiniWithGender {
    aerob_exercise: InternalExercise,
    gender: InternalGender,
}

#[wasm_bindgen]
pub struct EvaluatorMiniWithAgeGender {
    aerob_exercise: InternalExercise,
    age: u8,
    gender: InternalGender,
}

#[wasm_bindgen]
impl EvaluatorMini {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise) -> Self {
        Self { aerob_exercise: aerob_exercise.into() }
    }

    pub fn with_age(self, age: u8) -> EvaluatorMiniWithAge {
        EvaluatorMiniWithAge { aerob_exercise: self.aerob_exercise, age }
    }

    pub fn with_gender(self, gender: Gender) -> EvaluatorMiniWithGender {
        EvaluatorMiniWithGender { aerob_exercise: self.aerob_exercise, gender: gender.into() }
    }

    pub fn evaluate(&self, gender: Gender, age: u8, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32) -> Option<Evaluation> {
        let results = vec![jump, pushup, situp, torso];
        perform_evaluation(ChallengeType::HungarofitMini, self.aerob_exercise, gender.into(), age, aerob_result, results)
    }
}

#[wasm_bindgen]
impl EvaluatorMiniWithAge {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise, age: u8) -> Self {
        Self { aerob_exercise: aerob_exercise.into(), age }
    }

    pub fn with_gender(self, gender: Gender) -> EvaluatorMiniWithAgeGender {
        EvaluatorMiniWithAgeGender { aerob_exercise: self.aerob_exercise, age: self.age, gender: gender.into() }
    }

    pub fn evaluate(&self, gender: Gender, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32) -> Option<Evaluation> {
        let results = vec![jump, pushup, situp, torso];
        perform_evaluation(ChallengeType::HungarofitMini, self.aerob_exercise, gender.into(), self.age, aerob_result, results)
    }
}

#[wasm_bindgen]
impl EvaluatorMiniWithGender {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise, gender: Gender) -> Self {
        Self { aerob_exercise: aerob_exercise.into(), gender: gender.into() }
    }

    pub fn with_age(self, age: u8) -> EvaluatorMiniWithAgeGender {
        EvaluatorMiniWithAgeGender { aerob_exercise: self.aerob_exercise, gender: self.gender, age }
    }

    pub fn evaluate(&self, age: u8, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32) -> Option<Evaluation> {
        let results = vec![jump, pushup, situp, torso];
        perform_evaluation(ChallengeType::HungarofitMini, self.aerob_exercise, self.gender, age, aerob_result, results)
    }
}

#[wasm_bindgen]
impl EvaluatorMiniWithAgeGender {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise, age: u8, gender: Gender) -> Self {
        Self { aerob_exercise: aerob_exercise.into(), age, gender: gender.into() }
    }

    pub fn evaluate(&self, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32) -> Option<Evaluation> {
        let results = vec![jump, pushup, situp, torso];
        perform_evaluation(ChallengeType::HungarofitMini, self.aerob_exercise, self.gender, self.age, aerob_result, results)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluator_states() {
        let evaluator = Evaluator::new(Exercise::AerobRun2Km);
        
        // Full arguments
        let result1 = evaluator.evaluate(Gender::Male, 12, 480.0, 2.0, 40.0, 50.0, 8.0, 14.0, 10.0);
        assert!(result1.is_some());

        let evaluator_with_age = evaluator.with_age(12);
        let result2 = evaluator_with_age.evaluate(Gender::Male, 480.0, 2.0, 40.0, 50.0, 8.0, 14.0, 10.0);
        assert!(result2.is_some());

        // Need to re-create the base evaluator as it was moved
        let evaluator = Evaluator::new(Exercise::AerobRun2Km);
        let evaluator_with_gender = evaluator.with_gender(Gender::Male);
        let result3 = evaluator_with_gender.evaluate(12, 480.0, 2.0, 40.0, 50.0, 8.0, 14.0, 10.0);
        assert!(result3.is_some());

        // We can chain from the with_age evaluator
        let evaluator_with_age_gender = Evaluator::new(Exercise::AerobRun2Km).with_age(12).with_gender(Gender::Male);
        let result4 = evaluator_with_age_gender.evaluate(480.0, 2.0, 40.0, 50.0, 8.0, 14.0, 10.0);
        assert!(result4.is_some());
    }

    #[test]
    fn test_evaluator_mini_states() {
        let evaluator = EvaluatorMini::new(Exercise::AerobRun2Km);
        
        let result1 = evaluator.evaluate(Gender::Male, 12, 480.0, 2.0, 40.0, 50.0, 10.0);
        assert!(result1.is_some());
        
        let eval = result1.unwrap();
        assert_eq!(eval.motor_scores().len(), 4);

        let evaluator_with_age_gender = EvaluatorMini::new(Exercise::AerobRun2Km).with_age(12).with_gender(Gender::Male);
        let result2 = evaluator_with_age_gender.evaluate(480.0, 2.0, 40.0, 50.0, 10.0);
        assert!(result2.is_some());
        assert_eq!(result2.unwrap().motor_scores().len(), 4);
    }
}
