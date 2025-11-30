use clap::{Parser, Subcommand};
use evaluator::{load_tables, ChallengeType, Evaluator, Exercise, Gender};

#[derive(Parser)]
#[command(name = "evaluator")]
#[command(about = "Exercise Score Evaluator")]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Evaluate a single exercise
    Exercise {
        /// Exercise name (e.g., jump, pushup, aerob-run-2km)
        #[arg(short, long)]
        exercise: String,

        /// Gender: m/male or f/female
        #[arg(short, long)]
        gender: String,

        /// Age
        #[arg(short, long)]
        age: u8,

        /// Result value
        #[arg(short, long)]
        result: f32,

        /// Challenge context (hungarofit or hungarofitmini) - required for motor exercises
        #[arg(short, long)]
        challenge: Option<String>,
    },

    /// Evaluate a complete challenge
    Challenge {
        /// Challenge type: hungarofit or hungarofitmini
        #[arg(short = 't', long)]
        challenge_type: String,

        /// Gender: m/male or f/female
        #[arg(short, long)]
        gender: String,

        /// Age
        #[arg(short, long)]
        age: u8,

        /// Exercise results in format: exercise:result (e.g., jump:2.45)
        #[arg(short, long, num_args = 1..)]
        results: Vec<String>,
    },
}

fn main() {
    let args = Args::parse();

    let data = include_bytes!("../generated_tables.bin");
    let tables = load_tables(data);
    let evaluator = Evaluator::new(&tables);

    match args.command {
        Commands::Exercise {
            exercise,
            gender,
            age,
            result,
            challenge,
        } => {
            let gender = parse_gender(&gender);
            let exercise = match exercise.parse::<Exercise>() {
                Ok(ex) => ex,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };

            let challenge_context = challenge.as_ref().map(|c| parse_challenge_type(c));

            match evaluator.evaluate(exercise, gender, age, result, challenge_context) {
                Ok(score) => {
                    println!("Score: {:.2}", score);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Challenge {
            challenge_type,
            gender,
            age,
            results,
        } => {
            let gender = parse_gender(&gender);
            let challenge_type = parse_challenge_type(&challenge_type);

            let parsed_results: Vec<(Exercise, f32)> = results
                .iter()
                .map(|r| parse_exercise_result(r))
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_else(|e| {
                    eprintln!("Error parsing results: {}", e);
                    std::process::exit(1);
                });

            match evaluator.evaluate_challenge(challenge_type, gender, age, &parsed_results) {
                Ok(challenge_result) => {
                    println!("\n=== Challenge Results ===");
                    println!("Challenge: {:?}", challenge_type);
                    println!("Age: {}, Gender: {}", age, gender);
                    println!("\nIndividual Scores:");
                    for (exercise, score) in &challenge_result.exercise_scores {
                        println!("  {:<20} {:.2}", format!("{:?}", exercise), score);
                    }
                    println!("\nMotor Score:  {:.2}", challenge_result.motor_score());
                    println!("Aerob Score:  {:.2}", challenge_result.aerob_score());
                    println!("─────────────────────");
                    println!("Total Score:  {:.2}", challenge_result.total_score);
                    println!("Classification: {} ({:.2} - {:.2})", 
                        challenge_result.classification,
                        challenge_result.classification.score_range().0,
                        challenge_result.classification.score_range().1
                    );
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn parse_gender(gender: &str) -> Gender {
    match gender.parse::<Gender>() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn parse_challenge_type(challenge: &str) -> ChallengeType {
    match challenge.to_lowercase().as_str() {
        "hungarofit" | "motor6" => ChallengeType::Hungarofit,
        "hungarofitmini" | "motor4" => ChallengeType::HungarofitMini,
        _ => {
            eprintln!(
                "Invalid challenge type '{}'. Use 'hungarofit' or 'hungarofitmini'",
                challenge
            );
            std::process::exit(1);
        }
    }
}

fn parse_exercise_result(input: &str) -> Result<(Exercise, f32), String> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid format '{}'. Use exercise:result (e.g., jump:2.45)",
            input
        ));
    }

    let exercise = parts[0].parse::<Exercise>()?;
    let result = parts[1]
        .parse::<f32>()
        .map_err(|_| format!("Invalid result value: {}", parts[1]))?;

    Ok((exercise, result))
}
