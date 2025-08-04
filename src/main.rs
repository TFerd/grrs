use std::{collections::VecDeque, env::args, path::Path, process::ExitCode};

fn main() -> ExitCode {
    let mut args = args().skip(1);

    let mut inputs = VecDeque::<String>::new();
    let mut verbose = false;
    let mut output: Option<String> = None;
    let mut exitFlag = false;

    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => {
                help();
                exitFlag = true;
                break;
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

    if exitFlag == true {
        return ExitCode::SUCCESS;
    }

    if inputs.len() > 1 {
        // do stuff im stuff
        let search_query = inputs.pop_front().expect("unable to get search term");

        while let Some(input) = inputs.pop_front().as_deref() {
            let path = Path::new(input);

            if path.is_dir() {
                // do dir things
            } else if path.is_file() {
                // do file things
            }
        }
    } else {
        println!("Not enough parameters given. The correct syntax is 'grrs <query> [<files>]'");
        return ExitCode::SUCCESS;
    }

    println!("inputs: {:?}", inputs);

    ExitCode::SUCCESS
}

fn help() {
    println!("help...");
}
