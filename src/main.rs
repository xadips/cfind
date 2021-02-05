extern crate term;

use structopt::StructOpt;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let mut terminal = term::stdout().unwrap();
    terminal.fg(term::color::BRIGHT_GREEN).unwrap();
    let args = Cli::from_args();
    let file = File::open(&args.path).unwrap();
    let reader = BufReader::new(file);
    //let content = std::fs::read_to_string(&args.path).expect("could not read file");
    for (index, line) in reader.lines().enumerate()
    {
        let line = line.unwrap();
        if line.contains(&args.pattern) 
        {
            terminal.fg(term::color::RED).unwrap();
            print!("{}", index + 1);
            terminal.reset().unwrap();
            println!(": {}", line);
        }
    }
}
