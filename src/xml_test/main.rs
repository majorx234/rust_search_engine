use clap::{Arg, Command, Parser};
use std::fs::{self, File};
use std::io;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::exit;
use xml::reader::{EventReader, XmlEvent};

///simple prgramm to travers through xml documents
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// xml_file_path - file to parse through
    #[arg(short, long)]
    pub xml_file_path: String,
}

fn get_content_of_xml(xml_file_path: &Path) -> io::Result<String> {
    let xml_file = File::open(xml_file_path.clone()).unwrap_or_else(|err| {
        let error_msg = format!(
            "Error: could not open file {}: {}",
            xml_file_path.display(),
            err
        );
        // eprintln!(error_msg);
        Error::new(ErrorKind::Other, error_msg);
        exit(-1);
    });

    let event_reader = EventReader::new(xml_file);
    let mut content = String::new();
    for event in event_reader.into_iter() {
        if let XmlEvent::Characters(text) = event.expect("ToDo") {
            content.push_str(&text);
        }
    }
    Ok(content)
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let xml_dir_path = PathBuf::from(args.xml_file_path);
    let xml_files = fs::read_dir(xml_dir_path)?;
    for xml_file_path in xml_files {
        let xml_file_path = xml_file_path?.path();
        println!("file_path: {}", xml_file_path.display());
        if let Ok(content) = get_content_of_xml(&xml_file_path) {
            println!("{}", content);
        }
    }
    Ok(())
}
