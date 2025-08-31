use std::fs::{File, read_to_string};
use std::io::Write;
use std::{collections::VecDeque, env::args, path::Path, time::SystemTime};

use grrs::{big_help, handle_dir, handle_dir_recursive, log, print_matches, return_matches};

// a cursor traverses the file system and sends threads to work?
// a cursor spawns more cursors in dirs? doesnt make sense

// need to make sure:
// all threads are awaited
// awaiting waits for FUTURE threads (dont exit early until we're done traversing)
// i think this will be fine because we will be in the 'while' loop until we hit all dirs
// try using `thread::scope`

fn main() {
    let timer = SystemTime::now();
    let mut args = args().skip(1);

    let mut inputs = VecDeque::<String>::new();

    let mut verbose = false;
    let mut output: Option<String> = None;
    let mut recurse = false;

    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => {
                big_help();
                return;
            }
            "-v" | "--verbose" => verbose = true,
            "-r" | "--recursive" => recurse = true,
            "-o" | "--output" => {
                if let Some(output_path) = args.next() {
                    output = Some(output_path);
                } else {
                    panic!("No value specified for --output");
                }
            }
            _ => {
                if arg.starts_with('-') || arg.starts_with("--") {
                    println!("Unknown flag {}", arg);
                } else {
                    inputs.push_back(arg);
                }
            }
        }
    }

    if inputs.len() < 2 {
        println!("Not enough parameters given. The correct syntax is 'grrs <query> [<files>]'");
        return;
    }

    let search_query = inputs.pop_front().expect("unable to get search term");
    // let search_query: &'static str = search_query.clone().as_str();

    let mut output_file: Option<File> = None;

    if let Some(output) = output {
        output_file = Some(File::create(output).unwrap()); // TODO: make this a Arc<mutex<file>>
    }

    while let Some(input) = inputs.pop_front().as_deref() {
        let path = Path::new(input);

        log(format!("Checking path {:?}", path), verbose);

        if path.is_dir() {
            log(format!("Path {:?} is a directory", path), verbose);
            if recurse {
                rayon::scope(|s| handle_dir_recursive(&path, &search_query, &None, s));
            } else {
                handle_dir(&path, &search_query, None);
            }
        } else if path.is_file() {
            log(format!("Path {:?} is a file", path), verbose);

            let content = &read_to_string(path).unwrap_or_default();

            match output_file {
                Some(ref mut x) => {
                    log(format!("Writing to file {:?}", x), verbose);
                    let vec = return_matches(&search_query, content);

                    for line in vec {
                        x.write_all(line.as_bytes()).unwrap();
                        x.write_all(b"\n").unwrap();
                    }
                }
                None => {
                    print_matches(
                        &search_query,
                        content,
                        &std::io::stdout(),
                        path.to_str().unwrap(),
                    )
                    .unwrap();
                }
            }
        }
    }

    println!("grrs ran in {} ms", timer.elapsed().unwrap().as_millis());
}
