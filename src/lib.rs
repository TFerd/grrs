use std::{
    fs::{File, read_to_string},
    io::{Error, Write},
    path::Path,
    sync::mpsc::Sender,
};

use rayon::Scope;

const GREEN_TEXT: &'static str = "\x1b[0;32m";
const NORMAL_TEXT: &'static str = "\x1b[0m";

const UNABLE_TO_READ_DIR: &'static str = "Unable to read directory!";

pub fn handle_dir<D: AsRef<Path>>(
    dir: D,
    pattern: &str,
    output: Option<&File>,
) -> Result<(), Error> {
    for item in dir.as_ref().read_dir().expect(UNABLE_TO_READ_DIR) {
        let item = item.unwrap().path();

        if !item.is_dir() {
            handle_file(item, pattern, output).unwrap();
        }
    }

    Ok(())
}

// FIXME: unoptimal, could be faster. Try and remove clone()'s
// TODO: return a result? for why
pub fn handle_dir_recursive<'a, D: AsRef<Path>>(dir: D, pattern: &'a str, scope: &Scope<'a>) {
    for item in dir.as_ref().read_dir().expect(UNABLE_TO_READ_DIR) {
        let item_path = item.unwrap().path();

        if item_path.is_dir() {
            scope.spawn(move |s| {
                handle_dir_recursive(&item_path, pattern, s);
            });
        } else {
            let content = &read_to_string(item_path.clone()).unwrap_or_default();
            let filename = &item_path.to_str().unwrap();

            print_matches(&pattern, content, &std::io::stdout(), filename).unwrap();
        }
    }
}

pub fn handle_dir_recursive_with_output<'a, D: AsRef<Path>>(
    dir: D,
    pattern: &'a str,
    scope: &Scope<'a>,
    tx: Sender<String>,
) {
    for item in dir.as_ref().read_dir().expect(UNABLE_TO_READ_DIR) {
        let item_path = item.unwrap().path();

        if item_path.is_dir() {
            let tx_clone = tx.clone();
            scope.spawn(move |s| {
                handle_dir_recursive_with_output(&item_path, pattern, s, tx_clone);
            });
        } else {
            let content = &read_to_string(item_path.clone()).unwrap_or_default();
            let filename = &item_path.to_str().unwrap();

            for line in return_matches(pattern, content) {
                tx.send(format!("{}: {}", filename, line)).unwrap();
            }
        }
    }
}

/// Takes a `file` path input and either writes to a provided `output` file or prints the results to the terminal
/// if no such file is provided.
///
/// TODO: Allow developer to add their own writer here, aka add an `impl Write` param
pub fn handle_file<F: AsRef<Path>>(
    file: F,
    pattern: &str,
    output: Option<&File>,
) -> Result<(), Error> {
    let path = file.as_ref();
    let content = &read_to_string(path).unwrap_or_default();
    if let Some(mut output) = output {
        let vec = return_matches(pattern, content);

        for line in vec {
            output
                .write(format!("{}: {}\n", path.to_str().unwrap(), line).as_bytes())
                .unwrap();
        }
    } else {
        print_matches(
            pattern,
            content,
            &std::io::stdout(),
            &path.to_str().unwrap(),
        )
        .unwrap();
    }

    Ok(())
}

pub fn big_help() {
    println!("help...");
}
pub fn little_help() {}

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

/// Prints any occurences of `pattern` in `content` to the `writer`
///
/// TODO: make filename optional
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

pub fn log(message: String, verbose: bool) {
    if verbose == false {
        return;
    }

    writeln!(&std::io::stdout(), "{}", message).unwrap();
}

#[test]
fn handle_dir_should_throw() {}

#[test]
fn format_match_should_format() {}
