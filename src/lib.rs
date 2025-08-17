use std::io::{Error, Write};

const GREEN_TEXT: &str = "\x1b[0;32m";
const NORMAL_TEXT: &str = "\x1b[0m";

/// Prints any occurences of `pattern` in `content` to the `writer`
pub fn print_matches(pattern: &str, content: &str, mut writer: impl Write) -> Result<(), Error> {
    for line in content.lines() {
        if line.contains(pattern) {
            let match_index = line.find(pattern).unwrap();

            let pre_pattern_str = &line[0..(if match_index > 0 { match_index - 1 } else { 0 })];
            let post_pattern_str = &line[(match_index + pattern.len())..(line.len())];
            writeln!(
                writer,
                "{}{}{}{}{}",
                pre_pattern_str, GREEN_TEXT, pattern, NORMAL_TEXT, post_pattern_str
            )?;
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

#[test]
fn find_matches_finds_matches() {
    let mut writer = Vec::new();
    let result = print_matches(
        "pattern",
        "benis\nboosy\ni have a pattern\nwuuzy",
        &mut writer,
    );

    assert!(result.is_ok());
    assert_eq!(writer, b"i have a pattern\n");
}

#[test]
fn return_matches_returns_matches() {
    let result = return_matches("match", "hello\nmatchme\nim a match\nim a catch");

    assert_eq!(2, result.len());
    assert_eq!(*result.get(0).unwrap(), "matchme");
    assert_eq!(*result.get(1).unwrap(), "im a match");
}
