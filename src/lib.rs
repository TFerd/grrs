use std::{
    fs::{File, read_to_string},
    io::{Error, Write},
    path::Path,
    sync::Arc,
    thread::Scope,
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

// will be recursive // or no // or yes
pub fn handle_dir<'a, D: AsRef<Path>>(
    dir: D,
    pattern: &'a str,
    output: &'a Option<Arc<File>>,
    scope: &'a Scope<'a, 'a>,
) {
    // let dir = Arc::clone(&dir);
    // let pattern = Arc::clone(&pattern);

    for item in dir
        .as_ref()
        // .as_ref()
        .read_dir()
        .expect("Unable to read directory!")
    {
        let item_path = item.unwrap().path();

        if item_path.is_dir() {
            scope.spawn(move || {
                // let dir = dir.clone();
                handle_dir(&item_path, pattern, output, scope);
            });
        } else {
            let content = &read_to_string(item_path.clone()).unwrap();
            if let Some(mut output) = output.clone() {
                let vec = return_matches(&pattern, content);

                for line in vec {
                    output
                        .write(format!("{:?}:{}\n", &item_path, line).as_bytes())
                        .unwrap();
                }
            } else {
                print_matches(
                    &pattern,
                    content,
                    &std::io::stdout(),
                    &item_path.to_str().unwrap(),
                )
                .unwrap();
            }
        }
    }
}

/// Takes a `file` path input and either writes to a provided `output` file or prints the results to the terminal
/// if no such file is provided.
///
/// TODO: Allow developer to add their own writer here, aka add an `impl Write` param
/// i dont even need this
pub fn handle_file<F: AsRef<Path>>(file: F, pattern: &str, output: Option<File>) {
    let path = file.as_ref();
    let content = &read_to_string(path).unwrap_or_default();
    if let Some(mut output) = output {
        let vec = return_matches(pattern, content);

        for line in vec {
            output
                .write(format!("{:?}:{}\n", path, line).as_bytes())
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
