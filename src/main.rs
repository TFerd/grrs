use std::{collections::VecDeque, env::args, path::Path, time::SystemTime};

fn main() {
    let timer = SystemTime::now();
    let mut args = args().skip(1);

    let mut inputs = VecDeque::<String>::new();

    let mut verbose = false; // TODO: implement these jawns
    let mut output: Option<String> = None;

    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => {
                help();
                return;
            }
            "-v | --verbose" => verbose = true,
            "-r | --recursive" => println!("recursive not implemented"),
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
                // do dir things
                // if -recursive is set then do recursively
            } else if path.is_file() {
                // do file things
                // grrs::find_matches();
            }
        }
    } else {
        println!("Not enough parameters given. The correct syntax is 'grrs <query> [<files>]'");
        return;
    }

    println!("inputs: {:?}", inputs);
    println!("grrs ran in {} ms", timer.elapsed().unwrap().as_millis());
}

fn help() {
    println!("help...");
}
