pub mod exercise;
pub mod lookup;
pub mod classification;
pub mod evaluation_results;
pub mod gender;
pub mod format_decode;
pub mod evaluator;

// Re-export evaluator functions for convenience
#[cfg(target_arch = "wasm32")]
pub use evaluator::{evaluate, evaluate_mini};

#[cfg(test)]
pub mod lookup_test;
