use regex::RegexBuilder;
use std::fs::{self, DirEntry};
use std::io;
use std::io::Write;
use std::path::Path;

/**
 * This function takes a string slice and a pattern and writes all lines
 * containing the pattern to the writer.
 *
 * @param content The content to search
 * @param pattern The pattern to search for in the content
 * @param writer The writer to write the lines to
 *
 */
pub fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl Write,
    case_insensitive: bool,
    line_numbers: bool,
    only_matching: bool,
    invert_match: bool,
) {
    let regex = RegexBuilder::new(pattern)
        .case_insensitive(case_insensitive)
        .build()
        .unwrap();

    for (index, line) in content.lines().enumerate() {
        let contains = regex.is_match(line);
        if invert_match != contains {
            let line_to_print = if only_matching {
                regex.find(line).map_or("", |mat| mat.as_str())
            } else {
                line
            };

            if !line_to_print.is_empty() {
                if line_numbers {
                    let _ = writeln!(writer, "{}:{}", index + 1, line_to_print);
                } else {
                    let _ = writeln!(writer, "{}", line_to_print);
                }
            }
        }
    }
}

pub fn list_directory_contents(path: &Path, show_hidden: bool) -> io::Result<()> {
    let entries = fs::read_dir(path)?
        .filter_map(Result::ok) // Ignore any errors during iteration.
        .collect::<Vec<DirEntry>>();

    for entry in entries {
        let path = entry.path();

        // Skip hidden files unless show_hidden is true.
        if !show_hidden
            && path
                .file_name()
                .and_then(|name| name.to_str())
                .map_or(false, |name| name.starts_with('.'))
        {
            continue;
        }

        // display the path
        if path.is_dir() {
            println!("{}/", path.file_name().unwrap().to_str().unwrap());
        } else {
            println!("{}", path.file_name().unwrap().to_str().unwrap());
        }
    }
    Ok(())
}
