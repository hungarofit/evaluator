# Hungarofit Evaluator

## Core Types

**Gender**: Male or Female

**Exercise**: Individual exercise type (Jump, Pushup, Situp, Torso, ThrowDouble, ThrowSingle, and various aerobic exercises like AerobRun2Km, AerobSwim500M, etc.)

**Score**: Numeric score (0-20) assigned based on exercise result using lookup tables

**Classification**: Performance rating based on total score:
- Concerning: 0 - 20.49
- Weak: 20.5 - 40.49
- Mediocre: 40.5 - 60.49
- Average: 60.5 - 80.49
- Good: 80.5 - 100.49
- Excellent: 100.5 - 120.49
- Outstanding: 120.5 - 140

**Context Type**: Hungarofit (motor6) or HungarofitMini (motor4) - determines which table variant to use for motor exercises

## Evaluator

Core API: `evaluate(exercise, gender, age, result, context?)`
- Returns score for a single exercise
- Context parameter required for motor exercises (Jump, Pushup, Situp, Torso) to specify motor4 vs motor6 tables
- Context optional for aerobic exercises (only one table variant exists)

## Data Tables

- XLSX files stored in `data/` folder
- Each file contains two sheets: Female (first) and Male (second)
- Table structure:
  - Header row: age values
  - First column: score values
  - Grid cells: result thresholds for achieving that score at that age
