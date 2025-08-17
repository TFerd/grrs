use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use std::{collections::VecDeque, env::args, path::Path, time::SystemTime};

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
                help();
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

    let mut output_file: Option<File> = None;

    if let Some(output) = output {
        output_file = Some(File::create(output).unwrap());
    }

    while let Some(input) = inputs.pop_front().as_deref() {
        let path = Path::new(input);

        log(format!("Checking path {:?}", path), verbose);

        if path.is_dir() {
            log(format!("Path {:?} is a directory", path), verbose);
            let mut queue = VecDeque::<PathBuf>::new();
            queue.push_back(path.to_path_buf());
            while !queue.is_empty() {
                let next_dir = queue.pop_front().unwrap();

                log(format!("Checking {:?}", next_dir.as_path()), verbose);

                for next_dir_item in next_dir.read_dir().unwrap() {
                    let next_dir_item_path = next_dir_item.unwrap().path();

                    log(format!("Checking {:?}", next_dir_item_path), verbose);

                    if next_dir_item_path.is_dir() {
                        if recurse == true {
                            queue.push_back(next_dir_item_path.to_path_buf());
                        }
                    } else {
                        let content = &read_to_string(next_dir_item_path).unwrap_or_default();

                        match output_file {
                            Some(ref mut x) => {
                                log(format!("Writing to file {:?}", x), verbose);
                                let vec = grrs::return_matches(&search_query, content);

                                for line in vec {
                                    x.write_all(line.as_bytes()).unwrap();
                                }
                            }
                            None => {
                                grrs::print_matches(&search_query, content, &std::io::stdout())
                                    .unwrap();
                            }
                        }
                    }
                }
            }
        } else if path.is_file() {
            log(format!("Path {:?} is a file", path), verbose);

            let content = &read_to_string(path).unwrap_or_default();

            match output_file {
                Some(ref mut x) => {
                    log(format!("Writing to file {:?}", x), verbose);
                    let vec = grrs::return_matches(&search_query, content);

                    for line in vec {
                        x.write_all(line.as_bytes()).unwrap();
                        x.write_all(b"\n").unwrap();
                    }
                }
                None => {
                    grrs::print_matches(&search_query, content, &std::io::stdout()).unwrap();
                }
            }
        }
    }

    println!("grrs ran in {} ms", timer.elapsed().unwrap().as_millis());
}

fn help() {
    println!("help...");
}

fn log(message: String, verbose: bool) {
    if verbose == false {
        return;
    }

    write!(&std::io::stdout(), "{}", message).unwrap();
}
