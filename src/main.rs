mod cli;
mod parser;

fn main() {
    // Parse command line arguments
    let args = std::env::args();
    let filename = cli::parse_command_line_args(args).file;

    // Read whole source file to string
    let source = match std::fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            std::process::exit(1);
        }
    };

    println!("{}", crate::parser::parse(&source));
}
