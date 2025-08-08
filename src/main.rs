use std::fs::read_to_string;
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
            "-v | --verbose" => verbose = true,
            "-r | --recursive" => recurse = true,
            "-o | --output" => {
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
        // do stuff im stuff
        let search_query = inputs.pop_front().expect("unable to get search term");

        while let Some(input) = inputs.pop_front().as_deref() {
            let path = Path::new(input);

            if path.is_dir() {
                if recurse == true {
                    let queue = VecDeque::<&Path>::new();
                } else {
                    let mut contents = path.read_dir().unwrap();
                    while let Some(content) = contents.next() {
                        let file_content =
                            read_to_string(content.unwrap().path()).unwrap_or("".to_string());

                        grrs::find_matches(&search_query, &file_content, &mut std::io::stdout()).unwrap();
                    }
                }
                // do dir things
                // if -recursive is set then do recursively
            } else if path.is_file() {
                // do file things
                grrs::find_matches(&search_query, &read_to_string(path).unwrap(), &mut std::io::stdout()).unwrap();
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
