use half::f16;
use crate::exercise;

#[derive(Debug)]
pub struct Sheet {
    ages: Vec<u8>,
    scores: Vec<u8>,
    results: Vec<Vec<f32>>,
    ascending: bool,
}

impl Sheet {
    pub fn ages(&self) -> &[u8] {
        &self.ages
    }

    pub fn scores(&self) -> &[u8] {
        &self.scores
    }

    pub fn is_ascending(&self) -> bool {
        self.ascending
    }

    /// Get results for a specific age column
    pub fn results_for_age(&self, age_idx: usize) -> Option<&[f32]> {
        self.results.get(age_idx).map(|v| v.as_slice())
    }

    /// Find the score for a given age and result
    pub fn lookup(&self, age: u8, result: f32) -> Option<f32> {
        // Find the largest age that is <= the specified age
        let age_idx = match self.ages.binary_search(&age) {
            Ok(idx) => idx, // Exact match
            Err(idx) => {
                // No exact match. idx is where age would be inserted.
                // We want the largest age < specified age, which is at idx - 1
                if idx == 0 {
                    // All ages are greater than specified age
                    return None;
                }
                idx - 1
            }
        };
        let age_results = self.results.get(age_idx)?;

        if age_results.is_empty() {
            return None;
        }

        // Binary search for result based on ascending/descending order
        // Skip 0.0 values which represent missing data
        let score_idx = if self.ascending {
            // Ascending: higher result = better, find highest score where threshold <= result
            age_results
                .iter()
                .rposition(|&threshold| threshold > 0.0 && threshold <= result)
                .unwrap_or(0)
        } else {
            // Descending: lower result = better, find highest score where threshold >= result
            age_results
                .iter()
                .rposition(|&threshold| threshold > 0.0 && threshold >= result)
                .unwrap_or(0)
        };

        // If result is worse than the worst score, return 0
        if score_idx == 0 {
            let worst_threshold = age_results[0];
            // Also check that worst_threshold is valid (not 0.0 missing data)
            if worst_threshold > 0.0 && ((self.ascending && result < worst_threshold)
                || (!self.ascending && result > worst_threshold))
            {
                return Some(0.0);
            }
        }

        Some(self.scores[score_idx] as f32 / 2.0)
    }
}

#[derive(Debug)]
pub struct AllTables {
    sheets: Vec<Sheet>,
}

impl AllTables {
    /// Get sheets for an exercise by name
    /// Returns (girls_sheet, boys_sheet) if found
    pub fn get_exercise(&self, name: &str) -> Option<(&Sheet, &Sheet)> {
        // Find the index in the ordered exercises list
        let idx = exercise::Exercise::all_exercises_ordered()
            .iter()
            .enumerate()
            .find(|(_, (ex, _))| {
                let table_name = match ex {
                    exercise::Exercise::Jump | 
                    exercise::Exercise::Pushup | 
                    exercise::Exercise::Situp | 
                    exercise::Exercise::Torso => {
                        // For motor exercises, check both motor4 and motor6 variants
                        ex.table_name(Some(exercise::ChallengeType::Hungarofit)) == name
                            || ex.table_name(Some(exercise::ChallengeType::HungarofitMini)) == name
                    }
                    _ => ex.table_name(None) == name,
                };
                table_name
            })
            .map(|(i, _)| i)?;
        
        let girls_idx = idx * 2;
        let boys_idx = idx * 2 + 1;
        Some((&self.sheets[girls_idx], &self.sheets[boys_idx]))
    }

    /// Get sheet for an exercise by name and gender
    /// gender: true = male (fiúk), false = female (lányok)
    pub fn get_sheet(&self, name: &str, male: bool) -> Option<&Sheet> {
        // Find the index in the ordered exercises list
        let idx = exercise::Exercise::all_exercises_ordered()
            .iter()
            .enumerate()
            .find(|(_, (ex, _))| {
                let table_name = match ex {
                    exercise::Exercise::Jump | 
                    exercise::Exercise::Pushup | 
                    exercise::Exercise::Situp | 
                    exercise::Exercise::Torso => {
                        // For motor exercises, check both motor4 and motor6 variants
                        ex.table_name(Some(exercise::ChallengeType::Hungarofit)) == name
                            || ex.table_name(Some(exercise::ChallengeType::HungarofitMini)) == name
                    }
                    _ => ex.table_name(None) == name,
                };
                table_name
            })
            .map(|(i, _)| i)?;
        
        let sheet_idx = idx * 2 + if male { 1 } else { 0 };
        self.sheets.get(sheet_idx)
    }

    /// Get all sheets (for backwards compatibility)
    pub fn sheets(&self) -> &[Sheet] {
        &self.sheets
    }
}

pub fn load_tables(data: &[u8]) -> AllTables {
    let mut i = 0;
    let sheet_count = u32::from_le_bytes(data[i..i + 4].try_into().unwrap()) as usize;
    i += 4;

    let mut sheets = Vec::with_capacity(sheet_count);

    for _ in 0..sheet_count {
        let age_count = u16::from_le_bytes([data[i], data[i+1]]) as usize;
        i += 2;
        let score_count = u16::from_le_bytes([data[i], data[i+1]]) as usize;
        i += 2;
        let flags = data[i];
        i += 1;
        let ascending = (flags & 1) == 0;

        // Read ages (delta-decoded)
        let mut ages = Vec::with_capacity(age_count);
        let mut prev_age = 0u8;
        for _ in 0..age_count {
            let delta = data[i];
            i += 1;
            prev_age = prev_age.wrapping_add(delta);
            ages.push(prev_age);
        }

        // Read scores (raw u8 values)
        let scores = data[i..i + score_count].to_vec();
        i += score_count;

        // Read results grid (delta-decoded per column)
        let mut results = vec![Vec::with_capacity(score_count); age_count];
        for age_idx in 0..age_count {
            let mut prev_result = 0f32;
            for _ in 0..score_count {
                let delta_bits = u16::from_le_bytes([data[i], data[i + 1]]);
                i += 2;
                let delta = f16::from_bits(delta_bits).to_f32();
                prev_result += delta;
                results[age_idx].push(prev_result);
            }
        }

        sheets.push(Sheet {
            ages,
            scores,
            results,
            ascending,
        });
    }

    AllTables { sheets }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_ascending() {
        // Simulate: ages [4, 5], scores [2, 4, 6], ascending
        // Age 4: results [0.69, 0.72, 0.75]
        // Age 5: results [0.81, 0.84, 0.87]
        let sheet = Sheet {
            ages: vec![4, 5],
            scores: vec![2, 4, 6], // Represents scores 1.0, 2.0, 3.0
            results: vec![
                vec![0.69, 0.72, 0.75],
                vec![0.81, 0.84, 0.87],
            ],
            ascending: true,
        };

        // Age 4, result 0.72 should give score 2.0
        assert_eq!(sheet.lookup(4, 0.72), Some(2.0));
        // Age 4, result 0.74 should give score 2.0 (doesn't reach 0.75)
        assert_eq!(sheet.lookup(4, 0.74), Some(2.0));
        // Age 4, result 0.75 should give score 3.0
        assert_eq!(sheet.lookup(4, 0.75), Some(3.0));
        // Age 4, result 0.60 (worse than worst) should give 0
        assert_eq!(sheet.lookup(4, 0.60), Some(0.0));
        // Age 6 (not in table) should use age 5 data (largest age <= 6)
        // Age 5 thresholds: [0.81, 0.84, 0.87], so 0.70 is worse than worst
        assert_eq!(sheet.lookup(6, 0.70), Some(0.0));
    }

    #[test]
    fn test_lookup_descending() {
        // Simulate: ages [7, 8], scores [2, 4, 6], descending
        // Age 7: results [16.56, 16.39, 16.22] (lower is better)
        // Age 8: results [16.22, 16.05, 15.50]
        let sheet = Sheet {
            ages: vec![7, 8],
            scores: vec![2, 4, 6], // Represents scores 1.0, 2.0, 3.0
            results: vec![
                vec![16.56, 16.39, 16.22],
                vec![16.22, 16.05, 15.50],
            ],
            ascending: false,
        };

        // Age 7, result 16.39 should give score 2.0
        assert_eq!(sheet.lookup(7, 16.39), Some(2.0));
        // Age 7, result 16.30 should give score 2.0 (doesn't reach 16.22)
        assert_eq!(sheet.lookup(7, 16.30), Some(2.0));
        // Age 7, result 16.22 should give score 3.0
        assert_eq!(sheet.lookup(7, 16.22), Some(3.0));
        // Age 7, result 17.00 (worse than worst) should give 0
        assert_eq!(sheet.lookup(7, 17.00), Some(0.0));
    }

    #[test]
    fn test_lookup_invalid_age() {
        let sheet = Sheet {
            ages: vec![10, 11, 12],
            scores: vec![10, 20],
            results: vec![vec![1.0, 2.0], vec![1.1, 2.1], vec![1.2, 2.2]],
            ascending: true,
        };

        // Age 9: Less than minimum age, should return None
        assert_eq!(sheet.lookup(9, 1.5), None);
        // Age 13: Greater than maximum age, uses age 12 data (largest age <= 13)
        // Age 12 thresholds: [1.2, 2.2], result 1.5 qualifies for score 10/2.0 = 5.0
        assert_eq!(sheet.lookup(13, 1.5), Some(5.0));
    }

    #[test]
    fn test_lookup_out_of_bounds_ascending() {
        // Higher is better
        let sheet = Sheet {
            ages: vec![10],
            scores: vec![2, 4, 6], // 1.0, 2.0, 3.0
            results: vec![vec![10.0, 20.0, 30.0]],
            ascending: true,
        };

        // Way below min (0.0)
        assert_eq!(sheet.lookup(10, 5.0), Some(0.0));
        // Exact min match (1.0)
        assert_eq!(sheet.lookup(10, 10.0), Some(1.0));
        // Just above min (1.0)
        assert_eq!(sheet.lookup(10, 10.1), Some(1.0));
        
        // Way above max (should be max score 3.0)
        assert_eq!(sheet.lookup(10, 100.0), Some(3.0));
        // Exact max match
        assert_eq!(sheet.lookup(10, 30.0), Some(3.0));
    }

    #[test]
    fn test_lookup_out_of_bounds_descending() {
        // Lower is better (e.g. running time)
        let sheet = Sheet {
            ages: vec![10],
            scores: vec![2, 4, 6], // 1.0, 2.0, 3.0
            results: vec![vec![30.0, 20.0, 10.0]],
            ascending: false,
        };

        // Way above max time (worse than worst) -> 0.0
        assert_eq!(sheet.lookup(10, 35.0), Some(0.0));
        // Exact worst match -> 1.0
        assert_eq!(sheet.lookup(10, 30.0), Some(1.0));
        // Slightly better than worst -> 1.0
        assert_eq!(sheet.lookup(10, 29.9), Some(1.0));

        // Way below min time (better than best) -> max score 3.0
        assert_eq!(sheet.lookup(10, 5.0), Some(3.0));
        // Exact best match -> 3.0
        assert_eq!(sheet.lookup(10, 10.0), Some(3.0));
    }

    #[test]
    fn test_lookup_empty_results() {
        let sheet = Sheet {
            ages: vec![10],
            scores: vec![],
            results: vec![vec![]],
            ascending: true,
        };

        assert_eq!(sheet.lookup(10, 5.0), None);
    }

    #[test]
    fn test_lookup_synthetic_random() {
        // Construct a larger table
        // Ages: 10..=20
        // Scores: 0..=100 (raw), so 0.0 to 50.0
        // Result function: Age + (Score * 0.1)
        // Ascending
        
        let ages: Vec<u8> = (10..=20).collect();
        let scores: Vec<u8> = (0..=100).collect();
        let mut results = Vec::new();

        for &age in &ages {
            let mut col = Vec::new();
            for &score in &scores {
                let val = age as f32 + (score as f32 * 0.1);
                col.push(val);
            }
            results.push(col);
        }

        let sheet = Sheet {
            ages: ages.clone(),
            scores: scores.clone(),
            results,
            ascending: true,
        };

        // Test random points
        // Age 15, Score 50 (raw) -> 25.0 pts. Result should be 15 + 5.0 = 20.0
        assert_eq!(sheet.lookup(15, 20.0), Some(25.0));
        
        // Age 15, Result 20.05 (slightly better than 20.0, but not enough for next score 51 which is 20.1)
        // Threshold for score 50 is 20.0. Threshold for score 51 is 20.1.
        // 20.05 >= 20.0, so it qualifies for score 50.
        // It does NOT qualify for score 51 (requires 20.1).
        assert_eq!(sheet.lookup(15, 20.05), Some(25.0));

        // Age 20, Score 100 (raw) -> 50.0 pts. Result 20 + 10.0 = 30.0
        assert_eq!(sheet.lookup(20, 30.0), Some(50.0));

        // Age 10, Score 0 (raw) -> 0.0 pts. Result 10 + 0.0 = 10.0
        assert_eq!(sheet.lookup(10, 10.0), Some(0.0));
        
        // Age 10, Result 9.9 (worse than worst) -> 0.0 pts (special case 0.0 return)
        // Note: Score 0 is the first index.
        // The logic: score_idx = 0. worst_threshold = 10.0.
        // result (9.9) < worst (10.0) -> returns Some(0.0).
        // If we hit exact 10.0, score_idx=0, returns scores[0]/2.0 = 0.0.
        // So both return 0.0, but via different paths potentially.
        assert_eq!(sheet.lookup(10, 9.9), Some(0.0));
    }
}
