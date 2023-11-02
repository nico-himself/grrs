use regex::RegexBuilder;
use std::io::Write;

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

// let's create more function
// 1. case insensitive search
// 2. print line numbers
// 3. print only the matching part of the line
// 4. invert the search, i.e. print only the lines that do not match
