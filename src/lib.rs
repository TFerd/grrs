use std::{
    fs::{File, read_to_string},
    io::{Error, Write},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    thread::{self, JoinHandle},
};

const GREEN_TEXT: &'static str = "\x1b[0;32m";
const NORMAL_TEXT: &'static str = "\x1b[0m";

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

// will be recursive // or no
pub fn handle_dir(path: PathBuf, thread_master: Arc<Vec<JoinHandle<()>>>) {
    for item in path.read_dir().expect("Unable to read directory!") {
        let item_path = item.unwrap().path();

        if item_path.is_dir() {
            // thread rip
            thread::spawn(move || {
                handle_dir(item_path, Arc::clone(&thread_master));
            });
        } else {
        }
    }
}

/// Takes a `file` path input and either writes to a provided `output` file or prints the results to the terminal
/// if no such file is provided.
///
/// TODO: Allow developer to add their own writer here, aka add an `impl Write` param
pub fn handle_file<U: AsRef<Path>>(file: U, pattern: &str, output: Option<File>) {
    let path = file.as_ref();
    let content = &read_to_string(path).unwrap_or_default();
    if let Some(mut output) = output {
        let vec = return_matches(pattern, content);

        for line in vec {
            output.write(format!("{:?}:{}\n", path, line).as_bytes());
        }
    } else {
        print_matches(
            pattern,
            content,
            &std::io::stdout(),
            &path.to_str().unwrap(),
        );
    }
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

#[test]
fn handle_dir_should_throw() {}
