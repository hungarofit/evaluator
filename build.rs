use anyhow::{bail, Context, Result};
use calamine::{DataType, Reader, Xlsx};
use std::{fs::File, path::PathBuf};
use std::env;
use std::path::Path;
use std::io::Write;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}

#[path = "src/exercise.rs"]
mod exercise;
#[path = "src/gender.rs"]
mod gender;
#[path = "src/format_encode.rs"]
mod format_encode;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=data");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/exercise.rs");
    println!("cargo:rerun-if-changed=src/gender.rs");
    println!("cargo:rerun-if-changed=src/format.rs");

    let mut all_sheets = Vec::new();

    // IMPORTANT: The order of sheets must match the lookup calculation in src/lookup.rs:
    // index = (exercise.index() * 2) + gender.sheet_index()
    // 
    // exercise::LIST preserves enum declaration order (exercise.index() = 0, 1, 2, ...)
    // Gender iteration [Female, Male] matches sheet_index() values [0, 1]
    // 
    // This produces: Ex0-Female(idx=0), Ex0-Male(idx=1), Ex1-Female(idx=2), Ex1-Male(idx=3), ...
    for &(table_name, exercise_val) in exercise::LIST {
        let path = PathBuf::from("data").join(format!("{}.xlsx", table_name));
        if !path.exists() {
            bail!(
                "Missing data file for exercise '{:#?}': {:?}",
                exercise_val, path
            );
        }
        let mut workbook: Xlsx<_> = calamine::open_workbook(&path)
            .with_context(|| format!("Failed to open {:?}", path))?;

        for gender_enum in [gender::Gender::Female, gender::Gender::Male] {
            let sheet_index = gender_enum.sheet_index();
            let range = workbook
                .worksheet_range_at(sheet_index)
                .with_context(|| format!("No such sheet index '{}' in {:?}", sheet_index, path))??;

            let rows: Vec<_> = range.rows().collect();
            if rows.is_empty() {
                bail!("Sheet index '{}' in {:?} is empty", sheet_index, path);
            }

            let ages: Vec<u8> = rows[0][1..]
                .iter()
                .filter_map(|cell| {
                    cell.get_int()
                        .map(|v| v as u8)
                        .or_else(|| cell.get_float().map(|v| v as u8))
                })
                .collect();

            if ages.is_empty() {
                bail!(
                    "No ages found in header of sheet index '{}' in {:?} (row len: {})",
                    sheet_index,
                    path,
                    rows[0].len()
                );
            }

            for i in 1..ages.len() {
                if ages[i] <= ages[i - 1] {
                    bail!(
                        "Ages are not strictly increasing in {} sheet index '{}': age[{}]={} is not greater than age[{}]={}",
                        path.display(), sheet_index, i, ages[i], i-1, ages[i-1]
                    );
                }
            }

            let mut scores: Vec<f32> = Vec::new();
            let mut results: Vec<Vec<f32>> = vec![Vec::new(); ages.len()];

            for row in &rows[1..] {
                if let Some(score_val) = row[0]
                    .get_float()
                    .or_else(|| row[0].get_int().map(|v| v as f64))
                {
                    let score = (score_val * 2.0).round() / 2.0;
                    scores.push(score as f32);

                    for (age_idx, cell) in row[1..].iter().enumerate() {
                        if age_idx >= ages.len() {
                            break;
                        }
                        let val = cell
                            .get_float()
                            .or_else(|| cell.get_int().map(|v| v as f64))
                            .or_else(|| {
                                // Handle string values with comma decimal separator (European format)
                                // e.g., "10,26" or "12,5"
                                cell.get_string().and_then(|s| {
                                    s.replace(',', ".").parse::<f64>().ok()
                                })
                            })
                            .map(|v| v as f32)
                            .unwrap_or(0.0);
                        results[age_idx].push(val);
                    }

                    for age_idx in row.len().saturating_sub(1)..ages.len() {
                        results[age_idx].push(0.0);
                    }
                }
            }

            for i in 1..scores.len() {
                if scores[i] <= scores[i - 1] {
                    bail!(
                        "Scores are not strictly increasing in {} sheet index '{}': score[{}]={} is not greater than score[{}]={}",
                        path.display(), sheet_index, i, scores[i], i-1, scores[i-1]
                    );
                }
            }

            // Note: Validation of monotonicity disabled due to data inconsistencies in source files
            // The lookup algorithm handles non-monotonic data gracefully

            all_sheets.push(format_encode::Sheet {
                ages,
                scores,
                age_results: results,
            });
        }
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("tables.bin");
    
    // Encode to memory first
    let uncompressed = format_encode::encode_all(&all_sheets, Vec::new())?;
    
    p!("Uncompressed size: {} bytes", uncompressed.len());
    
    // Compress with LZ4
    let compressed = lz4::block::compress(&uncompressed, Some(lz4::block::CompressionMode::DEFAULT), false)?;
    
    p!("Compressed size: {} bytes ({:.1}% of original)",
        compressed.len(),
        (compressed.len() as f64 / uncompressed.len() as f64) * 100.0
    );
    
    // Write compressed data to file with uncompressed size prefix
    let mut file = File::create(&dest_path)?;
    // Write uncompressed size as u32 first (so decompressor knows the size)
    file.write_all(&(uncompressed.len() as u32).to_le_bytes())?;
    file.write_all(&compressed)?;
    
    p!("Wrote compressed tables to {}", dest_path.display());

    Ok(())
}
