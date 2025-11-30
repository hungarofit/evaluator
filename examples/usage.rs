use evaluator::{load_tables, ChallengeType, Evaluator, Exercise, Gender};

fn main() {
    // Load the evaluation tables
    let data = include_bytes!("../generated_tables.bin");
    let tables = load_tables(data);
    let evaluator = Evaluator::new(&tables);

    println!("=== Evaluator API Usage Examples ===\n");

    // Example 1: Evaluate a single exercise
    println!("1. Single Exercise Evaluation:");
    let jump_score = evaluator
        .evaluate(
            Exercise::Jump,
            Gender::Male,
            12,    // age
            2.45,  // result in meters
            Some(ChallengeType::Hungarofit),
        )
        .unwrap();
    println!("   Jump (2.45m, 12yo male, Motor6): {:.2} points\n", jump_score);

    // Example 2: Evaluate an aerob exercise
    println!("2. Aerob Exercise Evaluation:");
    let run_score = evaluator
        .evaluate(
            Exercise::AerobRun2Km,
            Gender::Female,
            14,    // age
            540.0, // result in seconds (9 minutes)
            None,
        )
        .unwrap();
    println!("   2km Run (9:00, 14yo female): {:.2} points\n", run_score);

    // Example 3: Complete Hungarofit (Motor6) Challenge
    println!("3. Complete Hungarofit Challenge:");
    let hungarofit_results = vec![
        (Exercise::Jump, 2.45),
        (Exercise::Pushup, 45.0),
        (Exercise::Situp, 55.0),
        (Exercise::ThrowDouble, 8.5),
        (Exercise::ThrowSingle, 15.2),
        (Exercise::Torso, 12.0),
        (Exercise::AerobRun2Km, 480.5),
    ];

    match evaluator.evaluate_challenge(
        ChallengeType::Hungarofit,
        Gender::Male,
        12,   // age
        &hungarofit_results,
    ) {
        Ok(result) => {
            println!("   Individual Scores:");
            for (exercise, score) in &result.exercise_scores {
                println!("     {:?}: {:.2}", exercise, score);
            }
            println!("\n   Motor Score: {:.2}", result.motor_score());
            println!("   Aerob Score: {:.2}", result.aerob_score());
            println!("   ─────────────────────");
            println!("   Total Score: {:.2}", result.total_score);
            println!("   Classification: {}\n", result.classification);
        }
        Err(e) => println!("   Error: {}\n", e),
    }

    // Example 4: Complete HungarofitMini (Motor4) Challenge
    println!("4. Complete HungarofitMini Challenge:");
    let hungarofit_mini_results = vec![
        (Exercise::Jump, 1.85),
        (Exercise::Pushup, 30.0),
        (Exercise::Situp, 40.0),
        (Exercise::Torso, 8.0),
        (Exercise::AerobRun1Mile, 420.0),
    ];

    match evaluator.evaluate_challenge(
        ChallengeType::HungarofitMini,
        Gender::Female,
        10,    // age
        &hungarofit_mini_results,
    ) {
        Ok(result) => {
            println!("   Total Score: {:.2}", result.total_score);
            println!("   Classification: {}\n", result.classification);
        }
        Err(e) => println!("   Error: {}\n", e),
    }

    // Example 5: Partial Challenge Evaluation
    println!("5. Partial Challenge (progress tracking):");
    let partial_results = vec![
        (Exercise::Jump, 2.0),
        (Exercise::Pushup, 35.0),
        (Exercise::Situp, 45.0),
    ];

    match evaluator.evaluate_partial_challenge(
        ChallengeType::Hungarofit,
        Gender::Male,
        12,
        &partial_results,
    ) {
        Ok(result) => {
            println!("   Completed exercises: {}", result.exercise_scores.len());
            println!("   Current total: {:.2}", result.total_score);
            println!("   (Note: Classification not meaningful for partial results)\n");
        }
        Err(e) => println!("   Error: {}\n", e),
    }

    // Example 6: Error handling - Invalid exercise for challenge type
    println!("6. Error Handling - Invalid Exercise:");
    let invalid_results = vec![
        (Exercise::Jump, 2.0),
        (Exercise::Pushup, 35.0),
        (Exercise::Situp, 45.0),
        (Exercise::Torso, 10.0),
        (Exercise::ThrowDouble, 8.0), // Not valid for Motor4!
        (Exercise::AerobRun2Km, 500.0),
    ];

    match evaluator.evaluate_challenge(
        ChallengeType::HungarofitMini,
        Gender::Male,
        12,
        &invalid_results,
    ) {
        Ok(_) => println!("   Unexpected success"),
        Err(e) => println!("   Expected error: {}\n", e),
    }

    // Example 7: Score classification
    println!("7. Score Classification:");
    for score in [15.0, 25.0, 45.0, 65.0, 85.0, 105.0, 125.0] {
        let classification = evaluator.classify_score(score);
        let range = classification.score_range();
        println!(
            "   Score {:.1} → {} ({:.2} - {:.2})",
            score, classification, range.0, range.1
        );
    }
}
