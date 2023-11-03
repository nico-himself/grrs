#![allow(unused)]

use anyhow::{Context, Result};
// `anyhow` provides a convenient wrapper around Rust's error types
use clap::{Arg, Parser}; // `clap` provides a convenient way for us to define a CLI
use std::io::BufRead; // `BufRead` provides a convenient way to read lines from a file

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(
    name = "mycli",
    about = "RustCLI · A simple command line interface written in Rust",
    version = "0.0" // until we release the first version
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Search for a pattern in a file and display the lines that contain it.
    Grep {
        pattern: String,          // The pattern to look for
        path: std::path::PathBuf, // The path to the file to read

        #[clap(short = 'i', long, help = "Case insensitive search")]
        case_insensitive: bool, // Whether to search case insensitively

        #[clap(short = 'n', long, help = "Display line numbers")]
        line_numbers: bool, // Whether to display line numbers

        #[clap(short = 'v', long = "invert-match", help = "Invert match")]
        invert_match: bool, // Whether to invert the matches, i.e., only show the lines that don't match

        #[clap(short = 'o', long, help = "Only matching part of the line")]
        only_matching: bool, // Whether to only show the part of the line that matches
    },
    /// List directory contents
    Ls {
        // ! #[clap(parse(from_os_str))]
        path: Option<std::path::PathBuf>,
        #[clap(short = 'a', long, help = "Show hidden files")]
        show_hidden: bool,
    },
}

fn handle_grep_command(
    pattern: String,
    path: std::path::PathBuf,
    case_insensitive: bool,
    line_numbers: bool,
    invert_match: bool,
    only_matching: bool,
) -> Result<()> {
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    // Call `find_matches` with the options provided by the subcommand
    rcli::find_matches(
        &content,
        &pattern,
        &mut std::io::stdout(),
        case_insensitive,
        line_numbers,
        only_matching,
        invert_match,
    );
    Ok(())
}

fn handle_ls_command(path: Option<std::path::PathBuf>, show_hidden: bool) -> Result<()> {
    let path = path.unwrap_or_else(|| std::path::PathBuf::from("."));
    rcli::list_directory_contents(&path, show_hidden)?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse(); // `clap` parses the CLI arguments for us

    match args.command {
        Commands::Grep {
            pattern,
            path,
            case_insensitive,
            line_numbers,
            invert_match,
            only_matching,
        } => handle_grep_command(
            pattern,
            path,
            case_insensitive,
            line_numbers,
            invert_match,
            only_matching,
        )?,
        Commands::Ls { path, show_hidden } => handle_ls_command(path, show_hidden)?,
    }

    Ok(())
}
