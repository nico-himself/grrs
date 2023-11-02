/**
 * This function takes a string slice and a pattern and writes all lines
 * containing the pattern to the writer.
 *
 * @param content The content to search
 * @param pattern The pattern to search for in the content
 * @param writer The writer to write the lines to
 *
 */
pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    // If the pattern is empty, write all lines to the writer
    if pattern.is_empty() {
        for line in content.lines() {
            if let Err(e) = writeln!(writer, "{}", line) {
                eprintln!("Error: {}", e);
            }
        }
    } else {
        // If the pattern is not empty, only write lines containing the pattern
        for line in content.lines() {
            if line.contains(pattern) {
                match writeln!(writer, "{}", line) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
    }
}
