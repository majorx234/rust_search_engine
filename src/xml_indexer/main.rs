use clap::{Arg, Command, Parser};
use search_engine::{lexer::Lexer, TermFreq, TermFreqIndex};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::exit;
use xml::reader::{EventReader, XmlEvent};

fn index_document(content: &str) -> TermFreq {
    let string_index = HashMap::new();
    return string_index;
}

///simple prgramm to travers through xml documents
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// xml_file_path - file to parse through
    #[arg(short, long)]
    pub xml_file_path: String,
    /// json_file_path - json file to save index
    #[arg(short, long)]
    pub json_file_path: String,
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
    // println!("file: {:?}", xml_file_path);
    for event in event_reader.into_iter() {
        if let XmlEvent::Characters(text) = event.expect("ToDo") {
            content.push_str(&text);
            content.push(' ');
        }
    }
    Ok(content)
}

fn main() -> io::Result<()> {
    let mut all_documents = TermFreqIndex::new();
    let mut tf_global = TermFreq::new();
    let args = Args::parse();
    let xml_dir_path = PathBuf::from(args.xml_file_path);
    let json_file_path = PathBuf::from(args.json_file_path);
    let xml_files = fs::read_dir(xml_dir_path)?;

    for xml_file_path in xml_files {
        let xml_file_path = xml_file_path?.path();
        if xml_file_path.is_dir() {
            continue;
        }
        println!("indexing: \"{}\"...", &xml_file_path.to_str().unwrap());
        let mut tf = TermFreq::new();
        if let Ok(content) = get_content_of_xml(&xml_file_path) {
            let char_content = content.chars().collect::<Vec<_>>();
            // println!("}{}", content);
            for token in Lexer::new(&char_content) {
                let token = token
                    .iter()
                    .map(|x| x.to_ascii_uppercase())
                    .collect::<String>();
                tf.entry(token.clone())
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
                tf_global
                    .entry(token)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }

            all_documents.insert(xml_file_path, tf);
        }
    }
    // saving index to json
    println!("Saving {}", json_file_path.to_str().unwrap());
    let index_file = File::create(json_file_path)?;
    serde_json::to_writer(index_file, &all_documents).expect("serde works");

    for (path, tf) in all_documents {
        println!(
            "File: {} has {} unique tokens",
            path.to_str().unwrap(),
            tf.len()
        )
    }

    let mut stats = tf_global.iter().collect::<Vec<_>>();
    stats.sort_by_key(|(_, f)| *f);
    stats.reverse();
    for entry in stats.iter().take(10) {
        println!("{} => {}", entry.0, entry.1);
    }

    Ok(())
}