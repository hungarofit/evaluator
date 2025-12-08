use crate::{exercise::Exercise, gender::Gender, lookup::lookup_internal};

macro_rules! assert_lookup_cases {
    ($exercise:expr, $gender:expr, $age:expr, $cases:expr) => {
        for [result, expected_score] in $cases {
            let score = lookup_internal($exercise, $gender, $age, *result as f32);
            assert_eq!(score.unwrap(), *expected_score as f32);
        }
    };
}

#[test]
fn test_lookup_motor4_jump() {
    let cases = [
        [0.0, 0.0],
        [1.00, 0.0],
        [1.33, 0.0],
        [1.34, 1.0],
        [1.57, 8.0],
        [1.58, 9.0],
        [1.98, 21.0],
        [9.99, 21.0],
    ];
    assert_lookup_cases!(Exercise::Motor4Jump, Gender::Male, 10, &cases);
}

#[test]
fn test_lookup_motor6_jump() {
    let cases = [
        [0.0, 0.0],
        [1.00, 0.0],
        [1.33, 0.0],
        [1.34, 0.5],
        [1.57, 4.0],
        [1.58, 4.5],
        [1.98, 10.5],
        [9.99, 10.5],
    ];
    assert_lookup_cases!(Exercise::Motor6Jump, Gender::Male, 10, &cases);
}

#[test]
fn test_lookup_motor6_throwdouble() {
    let cases = [
        [0.0, 0.0],
        [1.00, 0.0],
        [3.29, 0.0],
        [3.30, 0.5],
        [7.14, 7.5],
        [7.15, 8.0],
        [8.40, 10.5],
        [9.99, 10.5],
    ];
    assert_lookup_cases!(Exercise::Motor6ThrowDouble, Gender::Male, 10, &cases);
}

#[test]
fn test_lookup_aerob_12min() {
    let cases = [
        [0.0, 0.0],
        [1000.0, 0.0],
        [1279.0, 0.0],
        [1280.0, 1.0],
        [1429.0, 7.0],
        [1430.0, 8.0],
        [2900.0, 77.0],
        [6000.0, 77.0],
    ];
    assert_lookup_cases!(Exercise::AerobRun12Min, Gender::Female, 10, &cases);
}

#[test]
fn test_lookup_aerob_2km() {
    let cases = [
        [0.0, 0.0],
        [99.99, 0.0],
        [16.34, 0.0],
        [16.33, 1.0],
        [15.01, 7.0],
        [15.00, 8.0],
        [7.49, 77.0],
        [1.00, 77.0],
    ];
    assert_lookup_cases!(Exercise::AerobRun2Km, Gender::Male, 10, &cases);
}

#[test]
fn test_lookup_age() {
    let score = lookup_internal(Exercise::AerobRun2Km, Gender::Male, 6, 20.2);
    assert_eq!(score.unwrap(), 1.0);
    let score = lookup_internal(Exercise::AerobRun2Km, Gender::Male, 6, 20.1);
    assert_eq!(score.unwrap(), 2.0);
    let score = lookup_internal(Exercise::AerobRun2Km, Gender::Male, 25, 12.34);
    assert_eq!(score.unwrap(), 1.0);
    let score = lookup_internal(Exercise::AerobRun2Km, Gender::Male, 25, 12.26);
    assert_eq!(score.unwrap(), 2.0);
    let score = lookup_internal(Exercise::AerobRun2Km, Gender::Male, 60, 15.41);
    assert_eq!(score.unwrap(), 1.0);
    let score = lookup_internal(Exercise::AerobRun2Km, Gender::Male, 60, 15.32);
    assert_eq!(score.unwrap(), 2.0);
}

#[test]
fn test_throwdouble_vs_throwsingle_comparison() {
    let throwdouble_score = lookup_internal(Exercise::Motor6ThrowDouble, Gender::Male, 10, 4.35);
    let throwsingle_score = lookup_internal(Exercise::Motor6ThrowSingle, Gender::Male, 10, 4.1);
    assert_eq!(throwdouble_score.unwrap(), 3.0);
    assert_eq!(throwsingle_score.unwrap(), 3.0);
}
