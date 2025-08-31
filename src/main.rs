use std::fs::{File, read_to_string};
use std::io::Write;
use std::{collections::VecDeque, env::args, path::Path, time::SystemTime};

use grrs::{
    big_help, handle_dir, handle_dir_recursive, handle_file, log, print_matches, return_matches,
};

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
    // let mut output: Option<String> = None;
    let mut output_file: Option<File> = None;
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
                    output_file = Some(File::create(output_path).unwrap());
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

    let pattern = inputs.pop_front().expect("unable to get search term");

    while let Some(input) = inputs.pop_front().as_deref() {
        let path = Path::new(input);

        log(format!("Checking path {:?}", path), verbose);

        if path.is_dir() {
            log(format!("Path {:?} is a directory", path), verbose);
            if recurse {
                rayon::scope(|s| handle_dir_recursive(&path, &pattern, &None, s));
            } else {
                handle_dir(&path, &pattern, &output_file);
            }
        } else if path.is_file() {
            log(format!("Path {:?} is a file", path), verbose);

            handle_file(path, &pattern, None);
        }
    }

    println!("grrs ran in {} ms", timer.elapsed().unwrap().as_millis());
}
