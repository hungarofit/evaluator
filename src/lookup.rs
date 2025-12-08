use crate::{exercise, format_decode, gender::Gender};
use std::sync::LazyLock;

pub static TABLES_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/tables.bin"));

pub static TABLES: LazyLock<Vec<format_decode::Sheet>> = LazyLock::new(|| {
    // First 4 bytes contain the uncompressed size
    let uncompressed_size = u32::from_le_bytes([
        TABLES_DATA[0],
        TABLES_DATA[1],
        TABLES_DATA[2],
        TABLES_DATA[3],
    ]) as i32;
    
    // Decompress the rest of the data
    let decompressed = lz4::block::decompress(&TABLES_DATA[4..], Some(uncompressed_size))
        .expect("Failed to decompress tables.bin");
    
    format_decode::decode_all(&decompressed).expect("Failed to decode tables.bin")
});

#[derive(Debug)]
pub enum LookupError {
    IndexOutOfBounds(usize, usize),
}

impl std::fmt::Display for LookupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IndexOutOfBounds(idx, max) => {
                write!(f, "Table index {} out of bounds (max {})", idx, max)
            }
        }
    }
}

impl std::error::Error for LookupError {}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn lookup(exercise: exercise::Exercise, gender: Gender, age: u8, result: f32) -> Result<f32, wasm_bindgen::JsError> {
    Ok(lookup_internal(exercise, gender, age, result)?)
}

pub fn lookup_internal(exercise: exercise::Exercise, gender: Gender, age: u8, result: f32) -> Result<f32, LookupError> {
    if result <= 0.0 {
        return Ok(0.0);
    }
    let idx = (exercise.index() * 2) + gender.sheet_index();

    let sheet = TABLES.get(idx).ok_or(LookupError::IndexOutOfBounds(idx, TABLES.len()))?;

    let age_idx = match sheet.ages.binary_search(&age) {
        Ok(idx) => idx,
        Err(idx) => {
            if idx == 0 {
                0
            } else if idx >= sheet.ages.len() {
                sheet.ages.len() - 1
            } else {
                idx - 1
            }
        }
    };

    const EPSILON: f32 = 0.001;
    let age_column = &sheet.age_results[age_idx];

    let score = if exercise.is_lower_better() {
        let idx = age_column.partition_point(|&r| r > result);
        if idx == 0 {
            if result <= age_column[0] {
                sheet.scores[0]
            } else {
                0.0
            }
        } else if idx >= age_column.len() {
            sheet.scores[age_column.len() - 1]
        } else {
            if result <= age_column[idx] + EPSILON {
                sheet.scores[idx]
            } else {
                sheet.scores[idx - 1]
            }
        }
    } else {
        let idx = age_column.partition_point(|&r| r <= result + EPSILON);
        
        if idx == 0 {
            0.0
        } else if idx >= age_column.len() {
            sheet.scores[age_column.len() - 1]
        } else {
            sheet.scores[idx - 1]
        }
    };

    Ok(score)
}
