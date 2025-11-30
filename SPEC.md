# Hungarofit Evaluator Specification

## Enums

### Gender

- Female
- Male

### ExerciseGroup

- Aerob (prefix: aerob-)
- Motor4 (prefix: motor4-)
- Motor6 (prefix: motor6-)

### Challenge

- Hungarofit (all Motor6 + one Aerob)
- HungarofitMini (all Motor4 + one Aerob)

## Tables Data

- evaluation tables are defined as XLSX files in (data/)[data/] folder
- each XLSX file has two sheets, first is Female second is Male
- sheets contain a 2d evaluation table
- sheet header is age (skipping first cell)
- sheet first column is output score
- sheet grid cells are the input results acheived for the exercise
