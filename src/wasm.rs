use wasm_bindgen::prelude::*;
use crate::{
    AllTables,
    load_embedded_tables,
    ChallengeType,
    Exercise,
    Gender,
    Evaluator as InternalEvaluator,
    Classification
};
use once_cell::sync::Lazy;
use wasm_bindgen::JsValue;
use std::convert::TryFrom;

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

static TABLES: Lazy<AllTables> = Lazy::new(|| load_embedded_tables());

// ============================================================================
// The result of a full evaluation for Hungarofit
// ============================================================================

#[wasm_bindgen]
#[derive(Debug)]
pub struct Evaluation {
    pub total_score: f32,
    pub classification: Classification,
    pub aerob_score: f32,
    pub jump_score: f32,
    pub pushup_score: f32,
    pub situp_score: f32,
    pub torso_score: f32,
    pub throw_double_score: f32,
    pub throw_single_score: f32,
}

#[wasm_bindgen]
impl Evaluation {
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> JsValue {
        let obj = createObject();
        setProperty(&obj, "total_score", &JsValue::from(self.total_score));
        setProperty(&obj, "classification", &JsValue::from(self.classification as u32));
        setProperty(&obj, "aerob_score", &JsValue::from(self.aerob_score));
        setProperty(&obj, "jump_score", &JsValue::from(self.jump_score));
        setProperty(&obj, "pushup_score", &JsValue::from(self.pushup_score));
        setProperty(&obj, "situp_score", &JsValue::from(self.situp_score));
        setProperty(&obj, "torso_score", &JsValue::from(self.torso_score));
        setProperty(&obj, "throw_double_score", &JsValue::from(self.throw_double_score));
        setProperty(&obj, "throw_single_score", &JsValue::from(self.throw_single_score));
        obj
    }
}

/// The result of a full evaluation for HungarofitMini
#[wasm_bindgen]
#[derive(Debug)]
pub struct EvaluationMini {
    pub total_score: f32,
    pub classification: Classification,
    pub aerob_score: f32,
    pub jump_score: f32,
    pub pushup_score: f32,
    pub situp_score: f32,
    pub torso_score: f32,
}

#[wasm_bindgen]
impl EvaluationMini {
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> JsValue {
        let obj = createObject();
        setProperty(&obj, "total_score", &JsValue::from(self.total_score));
        setProperty(&obj, "classification", &JsValue::from(self.classification as u32));
        setProperty(&obj, "aerob_score", &JsValue::from(self.aerob_score));
        setProperty(&obj, "jump_score", &JsValue::from(self.jump_score));
        setProperty(&obj, "pushup_score", &JsValue::from(self.pushup_score));
        setProperty(&obj, "situp_score", &JsValue::from(self.situp_score));
        setProperty(&obj, "torso_score", &JsValue::from(self.torso_score));
        obj
    }
}

// ============================================================================
// Internal Helper for Evaluation
// ============================================================================

fn perform_evaluation_hungarofit(
    aerob_exercise: Exercise,
    gender: Gender,
    age: u8,
    aerob_result: f32,
    results: Vec<f32>
) -> Option<Evaluation> {
    let evaluator = InternalEvaluator::new(&TABLES);
    let challenge_type = ChallengeType::Hungarofit;
    
    let aerob_score = evaluator.evaluate(aerob_exercise, gender, age, aerob_result, Some(challenge_type)).ok()?;
    let jump_score = evaluator.evaluate(Exercise::Jump, gender, age, results[0], Some(challenge_type)).ok()?;
    let pushup_score = evaluator.evaluate(Exercise::Pushup, gender, age, results[1], Some(challenge_type)).ok()?;
    let situp_score = evaluator.evaluate(Exercise::Situp, gender, age, results[2], Some(challenge_type)).ok()?;
    let torso_score = evaluator.evaluate(Exercise::Torso, gender, age, results[3], Some(challenge_type)).ok()?;
    let throw_double_score = evaluator.evaluate(Exercise::ThrowDouble, gender, age, results[4], Some(challenge_type)).ok()?;
    let throw_single_score = evaluator.evaluate(Exercise::ThrowSingle, gender, age, results[5], Some(challenge_type)).ok()?;
    
    let total_score = aerob_score + jump_score + pushup_score + situp_score + torso_score + throw_double_score + throw_single_score;
    let classification = Classification::from_score(total_score);
    
    Some(Evaluation {
        total_score,
        classification,
        aerob_score,
        jump_score,
        pushup_score,
        situp_score,
        torso_score,
        throw_double_score,
        throw_single_score,
    })
}

fn perform_evaluation_hungarofit_mini(
    aerob_exercise: Exercise,
    gender: Gender,
    age: u8,
    aerob_result: f32,
    results: Vec<f32>
) -> Option<EvaluationMini> {
    let evaluator = InternalEvaluator::new(&TABLES);
    let challenge_type = ChallengeType::HungarofitMini;
    
    let aerob_score = evaluator.evaluate(aerob_exercise, gender, age, aerob_result, Some(challenge_type)).ok()?;
    let jump_score = evaluator.evaluate(Exercise::Jump, gender, age, results[0], Some(challenge_type)).ok()?;
    let pushup_score = evaluator.evaluate(Exercise::Pushup, gender, age, results[1], Some(challenge_type)).ok()?;
    let situp_score = evaluator.evaluate(Exercise::Situp, gender, age, results[2], Some(challenge_type)).ok()?;
    let torso_score = evaluator.evaluate(Exercise::Torso, gender, age, results[3], Some(challenge_type)).ok()?;
    
    let total_score = aerob_score + jump_score + pushup_score + situp_score + torso_score;
    let classification = Classification::from_score(total_score);
    
    Some(EvaluationMini {
        total_score,
        classification,
        aerob_score,
        jump_score,
        pushup_score,
        situp_score,
        torso_score,
    })
}

// ============================================================================
// Hungarofit Evaluator (Motor6)
// ============================================================================

#[wasm_bindgen]
pub struct Evaluator {
    aerob_exercise: Exercise,
}

#[wasm_bindgen]
pub struct EvaluatorWithAge {
    aerob_exercise: Exercise,
    age: u8,
}

#[wasm_bindgen]
pub struct EvaluatorWithGender {
    aerob_exercise: Exercise,
    gender: Gender,
}

#[wasm_bindgen]
pub struct EvaluatorWithAgeGender {
    aerob_exercise: Exercise,
    age: u8,
    gender: Gender,
}

#[wasm_bindgen]
impl Evaluator {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise) -> Self {
        Self { aerob_exercise }
    }

    pub fn with_age(self, age: u8) -> EvaluatorWithAge {
        EvaluatorWithAge { aerob_exercise: self.aerob_exercise, age }
    }

    pub fn with_gender(self, gender: Gender) -> EvaluatorWithGender {
        EvaluatorWithGender { aerob_exercise: self.aerob_exercise, gender }
    }

    pub fn evaluate(&self, gender: Gender, age: u8, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32, throw_double: f32, throw_single: f32) -> Option<Evaluation> {
        let results = vec![jump, pushup, situp, torso, throw_double, throw_single];
        perform_evaluation_hungarofit(self.aerob_exercise, gender, age, aerob_result, results)
    }
}

#[wasm_bindgen]
impl EvaluatorWithAge {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise, age: u8) -> Self {
        Self { aerob_exercise, age }
    }

    pub fn with_gender(self, gender: Gender) -> EvaluatorWithAgeGender {
        EvaluatorWithAgeGender { aerob_exercise: self.aerob_exercise, age: self.age, gender }
    }
    
    pub fn evaluate(&self, gender: Gender, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32, throw_double: f32, throw_single: f32) -> Option<Evaluation> {
        let results = vec![jump, pushup, situp, torso, throw_double, throw_single];
        perform_evaluation_hungarofit(self.aerob_exercise, gender, self.age, aerob_result, results)
    }
}

#[wasm_bindgen]
impl EvaluatorWithGender {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise, gender: Gender) -> Self {
        Self { aerob_exercise, gender }
    }

    pub fn with_age(self, age: u8) -> EvaluatorWithAgeGender {
        EvaluatorWithAgeGender { aerob_exercise: self.aerob_exercise, gender: self.gender, age }
    }

    pub fn evaluate(&self, age: u8, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32, throw_double: f32, throw_single: f32) -> Option<Evaluation> {
        let results = vec![jump, pushup, situp, torso, throw_double, throw_single];
        perform_evaluation_hungarofit(self.aerob_exercise, self.gender, age, aerob_result, results)
    }
}

#[wasm_bindgen]
impl EvaluatorWithAgeGender {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise, age: u8, gender: Gender) -> Self {
        Self { aerob_exercise, age, gender }
    }

    pub fn evaluate(&self, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32, throw_double: f32, throw_single: f32) -> Option<Evaluation> {
        let results = vec![jump, pushup, situp, torso, throw_double, throw_single];
        perform_evaluation_hungarofit(self.aerob_exercise, self.gender, self.age, aerob_result, results)
    }
}

// ============================================================================
// HungarofitMini Evaluator (Motor4)
// ============================================================================

#[wasm_bindgen]
pub struct EvaluatorMini {
    aerob_exercise: Exercise,
}

#[wasm_bindgen]
pub struct EvaluatorMiniWithAge {
    aerob_exercise: Exercise,
    age: u8,
}

#[wasm_bindgen]
pub struct EvaluatorMiniWithGender {
    aerob_exercise: Exercise,
    gender: Gender,
}

#[wasm_bindgen]
pub struct EvaluatorMiniWithAgeGender {
    aerob_exercise: Exercise,
    age: u8,
    gender: Gender,
}

#[wasm_bindgen]
impl EvaluatorMini {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise) -> Self {
        Self { aerob_exercise }
    }

    pub fn with_age(self, age: u8) -> EvaluatorMiniWithAge {
        EvaluatorMiniWithAge { aerob_exercise: self.aerob_exercise, age }
    }

    pub fn with_gender(self, gender: Gender) -> EvaluatorMiniWithGender {
        EvaluatorMiniWithGender { aerob_exercise: self.aerob_exercise, gender }
    }

    pub fn evaluate(&self, gender: Gender, age: u8, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32) -> Option<EvaluationMini> {
        let results = vec![jump, pushup, situp, torso];
        perform_evaluation_hungarofit_mini(self.aerob_exercise, gender, age, aerob_result, results)
    }
}

#[wasm_bindgen]
impl EvaluatorMiniWithAge {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise, age: u8) -> Self {
        Self { aerob_exercise, age }
    }

    pub fn with_gender(self, gender: Gender) -> EvaluatorMiniWithAgeGender {
        EvaluatorMiniWithAgeGender { aerob_exercise: self.aerob_exercise, age: self.age, gender }
    }

    pub fn evaluate(&self, gender: Gender, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32) -> Option<EvaluationMini> {
        let results = vec![jump, pushup, situp, torso];
        perform_evaluation_hungarofit_mini(self.aerob_exercise, gender, self.age, aerob_result, results)
    }
}

#[wasm_bindgen]
impl EvaluatorMiniWithGender {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise, gender: Gender) -> Self {
        Self { aerob_exercise, gender }
    }

    pub fn with_age(self, age: u8) -> EvaluatorMiniWithAgeGender {
        EvaluatorMiniWithAgeGender { aerob_exercise: self.aerob_exercise, gender: self.gender, age }
    }

    pub fn evaluate(&self, age: u8, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32) -> Option<EvaluationMini> {
        let results = vec![jump, pushup, situp, torso];
        perform_evaluation_hungarofit_mini(self.aerob_exercise, self.gender, age, aerob_result, results)
    }
}

#[wasm_bindgen]
impl EvaluatorMiniWithAgeGender {
    #[wasm_bindgen(constructor)]
    pub fn new(aerob_exercise: Exercise, age: u8, gender: Gender) -> Self {
        Self { aerob_exercise, age, gender }
    }

    pub fn evaluate(&self, aerob_result: f32, jump: f32, pushup: f32, situp: f32, torso: f32) -> Option<EvaluationMini> {
        let results = vec![jump, pushup, situp, torso];
        perform_evaluation_hungarofit_mini(self.aerob_exercise, self.gender, self.age, aerob_result, results)
    }
}


// ============================================================================
// Exercise helper functions
// ============================================================================

/// Convert a string to an Exercise enum variant.
/// Returns the Exercise if successful, or throws an error if the string is invalid.
/// 
/// Supported formats include:
/// - Short names: "jump", "pushup", "situp", "torso"
/// - Motor variants: "motor4-jump", "motor6-jump", etc.
/// - Throw variants: "throwdouble", "throw-double", "motor6-throwdouble"
/// - Aerob exercises: "aerob-run-2km", "aerob-swim-500m", etc.
#[wasm_bindgen(js_name = exerciseFromString)]
pub fn exercise_from_string(s: &str) -> Result<Exercise, JsValue> {
    Exercise::try_from(s).map_err(|e| JsValue::from_str(&e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluator_states() {
        let evaluator = Evaluator::new(Exercise::AerobRun2Km);
        
        let result1 = evaluator.evaluate(Gender::Male, 12, 480.0, 2.0, 40.0, 50.0, 8.0, 14.0, 10.0);
        assert!(result1.is_some());
        let eval = result1.unwrap();
        assert!(eval.total_score > 0.0);
    }

    #[test]
    fn test_evaluator_mini_states() {
        let evaluator = EvaluatorMini::new(Exercise::AerobRun2Km);
        
        let result1 = evaluator.evaluate(Gender::Male, 12, 480.0, 2.0, 40.0, 50.0, 10.0);
        assert!(result1.is_some());
        
        let eval = result1.unwrap();
        assert!(eval.total_score > 0.0);
    }

    #[test]
    fn test_exercise_from_string() {
        assert_eq!(exercise_from_string("aerob-swim-500m").unwrap(), Exercise::AerobSwim500M);
        assert_eq!(exercise_from_string("jump").unwrap(), Exercise::Jump);
        assert_eq!(exercise_from_string("motor6-throwdouble").unwrap(), Exercise::ThrowDouble);
        assert!(exercise_from_string("invalid").is_err());
    }
}
