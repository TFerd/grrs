use std::io::{Write, stdout};

/// Prints any occurences of `pattern` in `content` to the `writer`
fn find_matches(pattern: &str, content: &str, mut writer: impl Write) {
    let penmg = "safd";
    writeln!(writer, "whoa");
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
