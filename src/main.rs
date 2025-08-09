use std::fs::read_to_string;
use std::path::PathBuf;
use std::{collections::VecDeque, env::args, path::Path, time::SystemTime};

fn main() {
    let timer = SystemTime::now();
    let mut args = args().skip(1);

    let mut inputs = VecDeque::<String>::new();

    let mut verbose = false; // TODO: implement these jawns
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
                    output = Some(output_path)
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

    if inputs.len() > 1 {
        let search_query = inputs.pop_front().expect("unable to get search term");
        let mut writer = std::io::stdout();

        while let Some(input) = inputs.pop_front().as_deref() {
            let path = Path::new(input);

            if path.is_dir() {
                if recurse == true {
                    let mut queue = VecDeque::<PathBuf>::new();
                    queue.push_back(path.to_path_buf());
                    while !queue.is_empty() {
                        let next_dir = queue.pop_front().unwrap();

                        for next_dir_item in next_dir.read_dir().unwrap() {
                            let next_dir_item_path = next_dir_item.unwrap().path();

                            if next_dir_item_path.is_dir() {
                                queue.push_back(next_dir_item_path.to_path_buf());
                            } else {
                                grrs::find_matches(
                                    &search_query,
                                    &read_to_string(next_dir_item_path).unwrap(),
                                    &writer,
                                )
                                .unwrap();
                            }
                        }
                    }
                    path.read_dir().unwrap();
                } else {
                    let mut contents = path.read_dir().unwrap();
                    while let Some(content) = contents.next() {
                        let file_content =
                            read_to_string(content.unwrap().path()).unwrap_or("".to_string());

                        grrs::find_matches(&search_query, &file_content, &writer).unwrap();
                    }
                }
            } else if path.is_file() {
                grrs::find_matches(
                    &search_query,
                    &read_to_string(path).unwrap(),
                    &mut std::io::stdout(),
                )
                .unwrap();
            }
        }
    } else {
        println!("Not enough parameters given. The correct syntax is 'grrs <query> [<files>]'");
        return;
    }

    println!("grrs ran in {} ms", timer.elapsed().unwrap().as_millis());
}

fn help() {
    println!("help...");
}
