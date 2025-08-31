use std::io::{Error, Write};

const GREEN_TEXT: &'static str = "\x1b[0;32m";
const NORMAL_TEXT: &'static str = "\x1b[0m";

/// Prints any occurences of `pattern` in `content` to the `writer`
pub fn print_matches(
    pattern: &str,
    content: &str,
    mut writer: impl Write,
    filename: &str,
) -> Result<(), Error> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}: {}", filename, format_match(pattern, line))?;
        }
    }

    Ok(())
}

/// Returns lines containing matches of `pattern` within `content`
pub fn return_matches<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    let mut vec = Vec::new();

    for line in content.lines() {
        if line.contains(pattern) {
            vec.push(line);
        }
    }

    vec
}

/// Returns a formatted string highlighting matches
fn format_match<'a>(pattern: &str, mut line: &'a str) -> String {
    line = line.trim();
    let mut formatted_string = String::new();

    while let Some(i) = line.find(pattern) {
        let (pre_match_str, match_onwards_str) = line.split_at(i);
        let (match_str, post_match_str) = match_onwards_str.split_at(pattern.len());
        formatted_string += pre_match_str;
        formatted_string += GREEN_TEXT;
        formatted_string += match_str;
        formatted_string += NORMAL_TEXT;

        line = post_match_str;
    }

    formatted_string += line;

    formatted_string
}
