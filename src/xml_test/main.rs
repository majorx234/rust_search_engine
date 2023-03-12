use clap::{Arg, Command, Parser};
use std::fs::File;
use std::path::PathBuf;
use std::process::exit;
use xml::reader::EventReader;

///simple prgramm to travers through xml documents
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// xml_file_path - file to parse through
    #[arg(short, long)]
    pub xml_file_path: String,
}

fn main() {
    let args = Args::parse();
    let xml_file_path = PathBuf::from(args.xml_file_path);
    println!("file: {}", xml_file_path.display());
    let xml_file = File::open(xml_file_path.clone()).unwrap_or_else(|err| {
        eprintln!(
            "Error: could not open file {}: {}",
            xml_file_path.display(),
            err
        );
        exit(-1)
    });

    let event_reader = EventReader::new(xml_file);
    for event in event_reader.into_iter() {
        println!("event: {:?}", event);
    }
}
