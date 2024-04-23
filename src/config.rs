//! Everything about program configuration.

use clap::{arg, command, Parser};

/// Program CLI configuration.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Path to the input file.
    #[arg()]
    pub input_file: String,
}
