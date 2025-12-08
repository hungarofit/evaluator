#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::classification::Classification;
#[cfg(target_arch = "wasm32")]
use crate::evaluation_results::{HungarofitEvaluation, HungarofitMiniEvaluation};
#[cfg(target_arch = "wasm32")]
use crate::gender::Gender;
#[cfg(target_arch = "wasm32")]
use crate::lookup;

use crate::exercise::Exercise;

// ============================================================================
// Helper Functions
// ============================================================================

/// Returns the list of motor6 exercises used in the full Hungarofit evaluation
pub fn hungarofit_exercise_list() -> Vec<Exercise> {
    vec![
        Exercise::Motor6Jump,
        Exercise::Motor6Pushup,
        Exercise::Motor6Situp,
        Exercise::Motor6Torso,
        Exercise::Motor6ThrowDouble,
        Exercise::Motor6ThrowSingle,
    ]
}

/// Returns the list of motor4 exercises used in the Hungarofit Mini evaluation
pub fn hungarofit_mini_exercise_list() -> Vec<Exercise> {
    vec![
        Exercise::Motor4Jump,
        Exercise::Motor4Pushup,
        Exercise::Motor4Situp,
        Exercise::Motor4Torso,
    ]
}

// ============================================================================
// Evaluator Functions
// ============================================================================

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "evaluate")]
pub fn evaluate(
    gender: Gender,
    age: u8,
    aerob_exercise: Exercise,
    aerob_result: f32,
    motor_jump: f32,
    motor_pushup: f32,
    motor_situp: f32,
    motor_torso: f32,
    motor_throwdouble: f32,
    motor_throwsingle: f32,
) -> Result<HungarofitEvaluation, JsError> {
    const MIN_AGE: u8 = 7;
    const MAX_AGE: u8 = 20;
    
    if age < MIN_AGE || age > MAX_AGE {
        return Err(JsError::new(&format!("Invalid age {} for this evaluation (min: {}, max: {})", age, MIN_AGE, MAX_AGE)));
    }

    let aerob_score = lookup::lookup_internal(aerob_exercise, gender, age, aerob_result)
        .map_err(|e| JsError::new(&format!("Lookup error: {}", e)))?;

    let jump_score = lookup::lookup_internal(Exercise::Motor6Jump, gender, age, motor_jump)
        .map_err(|e| JsError::new(&format!("Lookup error: {}", e)))?;
    let pushup_score = lookup::lookup_internal(Exercise::Motor6Pushup, gender, age, motor_pushup)
        .map_err(|e| JsError::new(&format!("Lookup error: {}", e)))?;
    let situp_score = lookup::lookup_internal(Exercise::Motor6Situp, gender, age, motor_situp)
        .map_err(|e| JsError::new(&format!("Lookup error: {}", e)))?;
    let torso_score = lookup::lookup_internal(Exercise::Motor6Torso, gender, age, motor_torso)
        .map_err(|e| JsError::new(&format!("Lookup error: {}", e)))?;
    let throwdouble_score = lookup::lookup_internal(Exercise::Motor6ThrowDouble, gender, age, motor_throwdouble)
        .map_err(|e| JsError::new(&format!("Lookup error: {}", e)))?;
    let throwsingle_score = lookup::lookup_internal(Exercise::Motor6ThrowSingle, gender, age, motor_throwsingle)
        .map_err(|e| JsError::new(&format!("Lookup error: {}", e)))?;

    let total_motor_score = jump_score + pushup_score + situp_score + torso_score + throwdouble_score + throwsingle_score;
    let total_score = aerob_score + total_motor_score;

    Ok(HungarofitEvaluation {
        classification: Classification::from_score(total_score),
        total_score,
        aerob_score,
        jump_score,
        pushup_score,
        situp_score,
        torso_score,
        throwdouble_score,
        throwsingle_score,
    })
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "evaluateMini")]
pub fn evaluate_mini(
    gender: Gender,
    age: u8,
    aerob_exercise: Exercise,
    aerob_result: f32,
    motor_jump: f32,
    motor_pushup: f32,
    motor_situp: f32,
    motor_torso: f32,
) -> Result<HungarofitMiniEvaluation, JsError> {
    const MIN_AGE: u8 = 4;
    const MAX_AGE: u8 = 99;
    
    if age < MIN_AGE || age > MAX_AGE {
        return Err(JsError::new(&format!("Invalid age {} for this evaluation (min: {}, max: {})", age, MIN_AGE, MAX_AGE)));
    }

    let aerob_score = lookup::lookup_internal(aerob_exercise, gender, age, aerob_result)
        .map_err(|e| JsError::new(&format!("Lookup error: {}", e)))?;

    let jump_score = lookup::lookup_internal(Exercise::Motor4Jump, gender, age, motor_jump)
        .map_err(|e| JsError::new(&format!("Lookup error: {}", e)))?;
    let pushup_score = lookup::lookup_internal(Exercise::Motor4Pushup, gender, age, motor_pushup)
        .map_err(|e| JsError::new(&format!("Lookup error: {}", e)))?;
    let situp_score = lookup::lookup_internal(Exercise::Motor4Situp, gender, age, motor_situp)
        .map_err(|e| JsError::new(&format!("Lookup error: {}", e)))?;
    let torso_score = lookup::lookup_internal(Exercise::Motor4Torso, gender, age, motor_torso)
        .map_err(|e| JsError::new(&format!("Lookup error: {}", e)))?;

    let total_motor_score = jump_score + pushup_score + situp_score + torso_score;
    let total_score = aerob_score + total_motor_score;

    Ok(HungarofitMiniEvaluation {
        classification: Classification::from_score(total_score),
        total_score,
        aerob_score,
        jump_score,
        pushup_score,
        situp_score,
        torso_score,
    })
}
