#![allow(unused)]

use anyhow::{Context, Result}; // `anyhow` provides a convenient wrapper around Rust's error types
use clap::{Arg, Parser}; // `clap` provides a convenient way for us to define a CLI
use std::io::BufRead; // `BufRead` provides a convenient way to read lines from a file

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,          // The pattern to look for
    path: std::path::PathBuf, // The path to the file to read

    #[clap(short = 'i', long)]
    case_insensitive: bool, // Whether to search case insensitively

    #[clap(short = 'n', long)]
    line_numbers: bool, // Whether to display line numbers

    #[clap(short = 'v', long = "invert-match")]
    invert_match: bool, // Whether to invert the matches, i.e., only show the lines that don't match

    #[clap(short = 'o', long)]
    only_matching: bool, // Whether to only show the part of the line that matches
}

fn main() -> Result<()> {
    let args = Cli::parse(); // `clap` parses the CLI arguments for us
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    // `grrs::find_matches` is defined in `src/lib.rs`
    grrs::find_matches(
        &content,
        &args.pattern,
        &mut std::io::stdout(),
        args.case_insensitive,
        args.line_numbers,
        args.only_matching,
        args.invert_match,
    );
    Ok(())
}
