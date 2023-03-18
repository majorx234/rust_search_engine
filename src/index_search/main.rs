use clap::{Arg, Command, Parser};
use search_engine;
use search_engine::model::{TermFreq, TermFreqPerDoc};
use serde_json;
use std::env::args;
use std::fs::File;
use std::io;
use std::path::PathBuf;

/// index search
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// json_file_path - json file to save index
    #[arg(short, long)]
    pub json_file_path: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let json_file_path = PathBuf::from(args.json_file_path);
    println!(
        "Reading {} index file...",
        &json_file_path.to_str().unwrap()
    );
    let json_file = File::open(&json_file_path)?;

    let tf_index: TermFreqPerDoc = serde_json::from_reader(json_file)?;
    println!(
        "{} contains {} files",
        json_file_path.to_str().unwrap(),
        tf_index.len()
    );
    Ok(())
}
