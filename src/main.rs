#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]

use std::convert::TryInto;
use clap::{Parser, Subcommand};
use evaluator::{load_tables, ChallengeType, Evaluator, EvaluatorError, Exercise, Gender};

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
    Exercise {
        #[arg(short, long)]
        exercise: String,

        #[arg(short, long)]
        gender: String,

        #[arg(short, long)]
        age: u8,

        #[arg(short, long)]
        result: f32,
    },
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
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
        } => {
            let gender: Gender = gender.as_str().try_into()
                .map_err(|e: String| format!("Invalid gender: {}. {}", gender, e))?;
            let exercise: Exercise = exercise.as_str().try_into()
                .map_err(|e: String| format!("Invalid exercise: {}. {}", exercise, e))?;

            let challenge_context = challenge
                .map(|c| parse_challenge_type(&c))
                .transpose()?;

            let score = evaluator
                .evaluate(exercise, gender, age, result, challenge_context)
                .map_err(|e: EvaluatorError| format!("Evaluation failed: {}", e))?;

            println!("Score: {:.2}", score);
        }
    }
    Ok(())
}

fn parse_challenge_type(challenge: &str) -> std::result::Result<ChallengeType, String> {
    match challenge.to_lowercase().as_str() {
        "hungarofit" | "motor6" => Ok(ChallengeType::Hungarofit),
        "hungarofitmini" | "motor4" => Ok(ChallengeType::HungarofitMini),
        _ => Err(format!(
            "Invalid challenge type '{}'. Use 'hungarofit' or 'hungarofitmini'",
            challenge
        )),
    }
}
