#![allow(unused)]

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(short = 'p', long = "pattern")]
    pattern: String,
    /// The path to the file to read
    #[arg(short = 'f', long = "file")]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("Pattern: {}", args.pattern);
    println!("Path: {}", args.path.display());
}


