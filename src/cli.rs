//! CLI argument definitions using clap.

use clap::Parser;

/// Interactive AI-powered feature engineering CLI.
#[derive(Parser, Clone, Debug)]
#[command(name = "autofeat", version, about)]
pub struct Cli {
    /// Path to the input dataset (CSV).
    #[arg(short = 'd', long = "dataset")]
    pub dataset: Option<String>,

    /// Path to a saved pipeline file to load.
    #[arg(short = 'p', long = "pipeline")]
    pub pipeline: Option<String>,

    /// Enable verbose logging.
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,
}
