use std::{collections::VecDeque, env::Args, path::PathBuf};

pub struct CliArgs {
    // Enables verbose mode
    pub verbose: bool,

    // Output file name
    pub output: Option<PathBuf>,

    // Source file name
    pub file: PathBuf,
}

const USAGE: &str = "\
Usage: chalk [OPTIONS] INPUT

OPTIONS:
    -h                    Display this message
    -v, --verbose         Enable verbose mode
    -o FILENAME           Write output to FILENAME
";

pub fn parse_command_line_args(mut args: Args) -> CliArgs {
    // The first arg is the program name, we don't need that
    args.next();
    let mut args: VecDeque<String> = args.collect();

    let mut ret: CliArgs = CliArgs {
        verbose: false,
        output: None,
        file: PathBuf::new(),
    };

    while args.len() > 0 {
        let arg = &args[0];

        match arg.as_str() {
            "--verbose" | "-v" => {
                ret.verbose = true;
                args.pop_front();
            }
            "--output" | "-o" => {
                args.pop_front();
                let output_file = args.get(0);
                if output_file.is_none() {
                    println!("{}", USAGE);
                    eprintln!("[ERROR]: No output file provided");
                    std::process::exit(1);
                }
                ret.output = Some(PathBuf::from(output_file.unwrap()));
                args.pop_front();
            }
            "--help" | "-h" => {
                println!("{}", USAGE);
                std::process::exit(0);
            }
            _ => {
                break;
            }
        }
    }

    if args.len() < 1 {
        println!("{}", USAGE);
        eprintln!("[ERROR]: No input file provided");
        std::process::exit(1);
    }

    ret.file = PathBuf::from(&args[0]);

    ret
}
