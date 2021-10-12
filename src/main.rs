use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Chalk Compiler")]
struct Opt {
    /// Enables verbose mode
    #[structopt(short, long)]
    verbose: bool,

    /// Output file name
    #[structopt(short, parse(from_os_str))]
    output: Option<PathBuf>,

    /// Source file name
    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    // Get command line arguments
    let opt = Opt::from_args();

    // Read whole source file to string
    let source = match std::fs::read_to_string(opt.file) {
        Ok(contents) => contents,
        Err(_) => {
            std::process::exit(1);
        }
    };

    println!("{}", source);
}
