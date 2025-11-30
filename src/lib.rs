pub mod challenge;
pub mod evaluator;
pub mod exercise;
pub mod gender;
pub mod loader;
pub mod tables;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

// Re-export main types for convenience
pub use challenge::{ChallengeResult, Classification};
pub use evaluator::{Evaluator, EvaluatorError};
pub use exercise::{ChallengeType, Exercise};
pub use gender::Gender;
pub use tables::{AllTables, Sheet, load_tables};

// Embed the generated tables binary at compile time
// This makes it available in the WASM output without needing external files
const EMBEDDED_TABLES: &[u8] = include_bytes!("../generated_tables.bin");

/// Load embedded tables (for WASM and embedded environments)
/// This function loads the tables that were embedded at compile time
pub fn load_embedded_tables() -> AllTables {
    load_tables(EMBEDDED_TABLES)
}
