use crate::tables::Sheet;

/// Look up the score for a given age and result in a sheet
/// 
/// Returns:
/// - `Some(score)` if the age is found in the sheet
///   - Returns 0.0 if the result is worse than the minimum threshold
///   - Returns appropriate score based on the result thresholds
/// - `None` if the age is not found in the sheet
pub fn lookup(sheet: &Sheet, age: u8, result: f32) -> Option<f32> {
    sheet.lookup(age, result)
}

#[cfg(test)]
mod tests {
    // Tests are in tables.rs where the Sheet implementation lives
}
