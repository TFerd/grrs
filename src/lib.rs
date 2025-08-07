use std::io::{Error, Write};

/// Prints any occurences of `pattern` in `content` to the `writer`
fn find_matches(pattern: &str, content: &str, mut writer: impl Write) -> Result<(), Error> {
    while let Some(line) = content.lines().next() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }

    Ok(())
}

/// Returns lines containing matches of `pattern` within `content`
fn return_matches<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    let mut vec = Vec::new();
    vec.push(content);
    vec
}

#[test]
fn tests_work() {
    assert_eq!(1, 1);
}

#[test]
fn erm() {
    let mut thing = Vec::new();
    find_matches("pattern", "content", &mut thing);

    assert!(true);
    assert_eq!(thing, b"whoa\n");
}
