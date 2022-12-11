#![allow(unused)]

use std::fs::read_to_string;

use anyhow::{Context, Result}; // for error
use clap::Parser; // for parsing the input 
/// Search for a pattern in a file and display the line that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let filepath = &(args.path).as_path().display().to_string();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read the file {}", filepath.to_string()))?;

    println!("{}" , content.to_string());
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
