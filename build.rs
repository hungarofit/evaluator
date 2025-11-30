use std::{fs, path::PathBuf};
use calamine::{DataType, Reader, Xlsx};
use half::f16;

#[path = "src/exercise.rs"]
mod exercise;

fn write_u8(buf: &mut Vec<u8>, v: u8) {
    buf.push(v);
}

fn write_f16(buf: &mut Vec<u8>, v: f32) {
    buf.extend_from_slice(&f16::from_f32(v).to_bits().to_le_bytes());
}

fn write_u32(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(&v.to_le_bytes());
}

fn write_u16(buf: &mut Vec<u8>, v: u16) {
    buf.extend_from_slice(&v.to_le_bytes());
}

fn main() {
    if std::env::var("BUILD_RS_DEBUG").is_ok() {
        eprintln!("*** build.rs is waiting for debugger on PID: {} ***", std::process::id());
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    println!("cargo:rerun-if-changed=data");
    println!("cargo:rerun-if-changed=src/exercise.rs");
    println!("cargo:rerun-if-changed=generated_tables.bin");

    let mut out = Vec::new();
    let mut all_sheets = Vec::new();

    for &(exercise, descending) in exercise::Exercise::all_exercises_ordered() {
        let exercise_name = match exercise {
            exercise::Exercise::Jump | 
            exercise::Exercise::Pushup | 
            exercise::Exercise::Situp | 
            exercise::Exercise::Torso => {
                // Determine if this is motor4 or motor6 based on position
                // First 4 motor exercises (indices 10-13) are motor4, next 6 (14-19) are motor6
                let all_exercises = exercise::Exercise::all_exercises_ordered();
                let current_idx = all_exercises.iter().position(|(ex, _)| ex == &exercise && *ex == exercise).unwrap_or(0);
                
                // Motor4: indices 10-13, Motor6: indices 14-19
                if current_idx >= 10 && current_idx < 14 {
                    // Motor4
                    match exercise {
                        exercise::Exercise::Jump => "motor4-jump",
                        exercise::Exercise::Pushup => "motor4-pushup",
                        exercise::Exercise::Situp => "motor4-situp",
                        exercise::Exercise::Torso => "motor4-torso",
                        _ => unreachable!(),
                    }
                } else {
                    // Motor6
                    match exercise {
                        exercise::Exercise::Jump => "motor6-jump",
                        exercise::Exercise::Pushup => "motor6-pushup",
                        exercise::Exercise::Situp => "motor6-situp",
                        exercise::Exercise::Torso => "motor6-torso",
                        _ => unreachable!(),
                    }
                }
            }
            _ => exercise.table_name(None),
        };
        
        let path = PathBuf::from("data").join(format!("{}.xlsx", exercise_name));
        
        if !path.exists() {
            panic!("Missing data file for exercise '{}': {:?}", exercise_name, path);
        }

        let mut workbook: Xlsx<_> = match calamine::open_workbook(&path) {
            Ok(wb) => wb,
            Err(e) => {
                eprintln!("Warning: Failed to open {:?}: {}", path, e);
                continue;
            }
        };

        for sheet_name in ["lányok", "fiúk"] {
            let range = match workbook.worksheet_range(sheet_name) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Warning: Failed to read sheet '{}' in {:?}: {}", sheet_name, path, e);
                    continue;
                }
            };

            let rows: Vec<_> = range.rows().collect();
            
            if rows.is_empty() {
                eprintln!("Warning: Sheet '{}' in {:?} is empty", sheet_name, path);
                continue;
            }

            // Extract ages from header row (skip A1)
            let ages: Vec<u8> = rows[0][1..]
                .iter()
                .filter_map(|cell| {
                    cell.get_int().map(|v| v as u8)
                        .or_else(|| cell.get_float().map(|v| v as u8))
                })
                .collect();

            if ages.is_empty() {
                eprintln!("Warning: No ages found in header of sheet '{}' in {:?} (row len: {})", sheet_name, path, rows[0].len());
                continue;
            }

            // Validate ages are strictly increasing
            for i in 1..ages.len() {
                if ages[i] <= ages[i-1] {
                    panic!(
                        "Ages are not strictly increasing in {} sheet '{}': age[{}]={} is not greater than age[{}]={}",
                        path.display(), sheet_name, i, ages[i], i-1, ages[i-1]
                    );
                }
            }

            // Extract scores and results grid
            let mut scores = Vec::new();
            let mut results = vec![Vec::new(); ages.len()];

            for row in &rows[1..] {
                if let Some(score_val) = row[0].get_float().or_else(|| row[0].get_int().map(|v| v as f64)) {
                    let score = (score_val * 2.0).round() as u8;
                    scores.push(score);

                    for (age_idx, cell) in row[1..].iter().enumerate() {
                        if age_idx >= ages.len() {
                            break;
                        }
                        let val = cell.get_float()
                            .or_else(|| cell.get_int().map(|v| v as f64))
                            .map(|v| v as f32)
                            .unwrap_or(0.0);
                        results[age_idx].push(val);
                    }
                    
                    // Fill remaining columns if row is shorter than header
                    for age_idx in row[1..].len()..ages.len() {
                        results[age_idx].push(0.0);
                    }
                }
            }

            if ages.len() > 65535 || scores.len() > 65535 {
                panic!("Sheet {} has too many rows/cols: {} ages, {} scores", sheet_name, ages.len(), scores.len());
            }

            // Validate scores are strictly increasing
            for i in 1..scores.len() {
                if scores[i] <= scores[i-1] {
                    panic!(
                        "Scores are not strictly increasing in {} sheet '{}': score[{}]={} is not greater than score[{}]={}",
                        path.display(), sheet_name, i, scores[i], i-1, scores[i-1]
                    );
                }
            }

            // Validate results are monotonically increasing/decreasing based on ascending flag
            // Skip validation when either value is 0.0 (represents missing/empty data)
            for age_idx in 0..results.len() {
                let age_results = &results[age_idx];
                for i in 1..age_results.len() {
                    // Skip validation if either value is 0.0 (missing data)
                    if age_results[i] == 0.0 || age_results[i-1] == 0.0 {
                        continue;
                    }
                    
                    if descending {
                        // For descending exercises (lower is better), results should decrease
                        if age_results[i] >= age_results[i-1] {
                            panic!(
                                "Results are not strictly decreasing (descending exercise) in {} sheet '{}' age column {} (age {}): result[{}]={} is not less than result[{}]={}",
                                path.display(), sheet_name, age_idx, ages[age_idx], i, age_results[i], i-1, age_results[i-1]
                            );
                        }
                    } else {
                        // For ascending exercises (higher is better), results should increase
                        if age_results[i] <= age_results[i-1] {
                            panic!(
                                "Results are not strictly increasing (ascending exercise) in {} sheet '{}' age column {} (age {}): result[{}]={} is not greater than result[{}]={}",
                                path.display(), sheet_name, age_idx, ages[age_idx], i, age_results[i], i-1, age_results[i-1]
                            );
                        }
                    }
                }
            }

            all_sheets.push((ages, scores, results, descending));
        }
    }

    write_u32(&mut out, all_sheets.len() as u32);

    for (ages, scores, results, descending) in &all_sheets {
        write_u16(&mut out, ages.len() as u16);
        write_u16(&mut out, scores.len() as u16);
        write_u8(&mut out, if *descending { 1 } else { 0 });

        // Write ages (delta-encoded)
        let mut prev_age = 0u8;
        for &age in ages {
            write_u8(&mut out, age - prev_age);
            prev_age = age;
        }

        // Write scores (raw u8 values = score * 2)
        for &score in scores {
            write_u8(&mut out, score);
        }

        // Write results grid (delta-encoded per column)
        for age_results in results {
            let mut prev_result = 0f32;
            for &result in age_results {
                write_f16(&mut out, result - prev_result);
                prev_result = result;
            }
        }
    }

    eprintln!("Binary size: {} bytes (uncompressed)", out.len());
    
    // Write the uncompressed binary to the project root
    let out_path = PathBuf::from("generated_tables.bin");
    fs::write(&out_path, &out).unwrap();
    
    // Also write to OUT_DIR for potential use in build scripts
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(&out_dir).join("generated_tables.bin");
    fs::write(&dest_path, &out).unwrap();
    
    eprintln!("Generated tables written to: {:?}", out_path.canonicalize().unwrap());
}
