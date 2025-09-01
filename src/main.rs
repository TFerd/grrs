use std::fs::File;
use std::{collections::VecDeque, env::args, path::Path, time::SystemTime};

use grrs::{big_help, handle_dir, handle_dir_recursive, handle_file, log};

fn main() {
    let timer = SystemTime::now();
    let mut args = args().skip(1);

    let mut inputs = VecDeque::<String>::new();

    let mut verbose = false;
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
                handle_dir(
                    &path,
                    &pattern,
                    if let Some(ref o) = output_file {
                        Some(o)
                    } else {
                        None
                    },
                )
                .unwrap();
            }
        } else if path.is_file() {
            log(format!("Path {:?} is a file", path), verbose);

            handle_file(
                path,
                &pattern,
                if let Some(ref o) = output_file {
                    Some(o)
                } else {
                    None
                },
            )
            .unwrap();
        }
    }

    println!("grrs ran in {} ms", timer.elapsed().unwrap().as_millis());
}
