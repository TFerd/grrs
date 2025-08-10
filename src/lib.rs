use std::io::{Error, Write};

/// Prints any occurences of `pattern` in `content` to the `writer`
pub fn find_matches(pattern: &str, content: &str, mut writer: impl Write) -> Result<(), Error> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
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
    let result = find_matches(
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
