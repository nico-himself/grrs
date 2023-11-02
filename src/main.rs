#![allow(unused)]

use anyhow::{Context, Result}; // `anyhow` provides a convenient wrapper around Rust's error types
use clap::Parser; // `clap` provides a convenient way for us to define a CLI
use std::io::BufRead; // `BufRead` provides a convenient way to read lines from a file

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,          // the pattern to search for
    path: std::path::PathBuf, // the path to the file to read
}

fn main() -> Result<()> {
    let args = Cli::parse(); // `clap` parses the CLI arguments for us
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    // `grrs::find_matches` is defined in `src/lib.rs`
    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
