#![allow(unused)]

use clap::Parser;
use std::io::BufRead;

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
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    
    let f = std::fs::File::open(&args.path).unwrap();
    let reader = std::io::BufReader::new(f);

    for line in reader.lines() {
        // if line.contains(&args.pattern) {
        //     println!("{}", line);
        // }
        match line {
            Ok(val) => {
                // println!("{}", val); 
                if val.contains(&args.pattern) {
                    println!("{}", val);
                }
            },
            Err(err) => {
               println!("An error has occurred reading the file:"); 
               println!("{}", err) 
            }
        }
    }
}


