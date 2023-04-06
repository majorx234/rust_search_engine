use clap::Parser;
use search_engine::lexer::Lexer;
use search_engine::model::{TermFreq, TermFreqPerDoc, TermIndex};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Error, ErrorKind};
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

fn get_content_of_txt(file_path: &Path) -> io::Result<String> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(err) => {
            let error_msg = format!(
                "Error: could not open file {}: {}",
                file_path.display(),
                err
            );
            // eprintln!(error_msg);
            Err(Error::new(ErrorKind::Other, error_msg))
        }
    }
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

    let event_reader = EventReader::new(BufReader::new(xml_file));
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

fn get_content_of_file(file_path: &Path) -> io::Result<String> {
    let extension = file_path
        .extension()
        .ok_or_else(|| {
            let error_msg = format!(
                "ERROR: can't detect file type of {} without extension",
                file_path.display()
            );
            Error::new(ErrorKind::Other, error_msg)
        })?
        .to_string_lossy();
    match extension.as_ref() {
        "xhtml" | "xml" => get_content_of_xml(file_path),
        // TODO: specialized parser for markdown files
        "txt" | "md" => get_content_of_txt(file_path),
        _ => {
            eprintln!(
                "ERROR: can't detect file type of {file_path}: unsupported extension {extension}",
                file_path = file_path.display(),
                extension = extension
            );
            Err(Error::new(ErrorKind::Other, "file type no known!"))
        }
    }
}

fn add_folder_to_index(xml_dir_path: PathBuf) -> Result<(TermIndex, usize), Box<Error>> {
    let mut skipped = 0;
    let mut all_documents = TermFreqPerDoc::new();
    let mut tf_global = TermFreq::new();

    let mut folder_stack = Vec::new();
    folder_stack.push(xml_dir_path);

    while !folder_stack.is_empty() {
        if let Some(current_folder) = folder_stack.pop() {
            let files = fs::read_dir(current_folder)?;
            for file_path in files {
                let file_path = file_path?.path();
                if file_path.is_dir() {
                    folder_stack.push(file_path);
                    continue;
                }
                println!("indexing: \"{}\"...", &file_path.to_str().unwrap());
                let mut tf = TermFreq::new();
                let mut nterm = 0;
                if let Ok(content) = get_content_of_file(&file_path) {
                    let char_content = content.chars().collect::<Vec<_>>();
                    // println!("}{}", content);
                    for token in Lexer::new(&char_content) {
                        tf.entry(token.clone())
                            .and_modify(|counter| *counter += 1)
                            .or_insert(1);
                        tf_global
                            .entry(token)
                            .and_modify(|counter| *counter += 1)
                            .or_insert(1);
                        nterm += 1;
                    }

                    all_documents.insert(file_path, (nterm, tf));
                } else {
                    skipped *= 1;
                    continue;
                }
            }
        }
    }
    let term_index = TermIndex {
        term_freq_per_doc: all_documents,
        doc_freq: tf_global,
    };
    return Ok((term_index, skipped));
}

fn save_index_as_json(term_index: &TermIndex, json_file_path: &PathBuf) -> io::Result<()> {
    // saving index to json
    println!("Saving {}", json_file_path.to_str().unwrap());
    let index_file = File::create(json_file_path)?;
    serde_json::to_writer(BufWriter::new(index_file), &term_index).expect("serde works");
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let json_file_path = PathBuf::from(args.json_file_path);
    let xml_dir_path = PathBuf::from(args.xml_file_path);

    if let Ok((term_index, skipped)) = add_folder_to_index(xml_dir_path) {
        save_index_as_json(&term_index, &json_file_path)?;
        for (path, (n, tf)) in term_index.term_freq_per_doc {
            println!(
                "File: {} has {} unique tokens",
                path.to_str().unwrap(),
                tf.len()
            )
        }

        let mut stats = term_index.doc_freq.iter().collect::<Vec<_>>();
        stats.sort_by_key(|(_, f)| *f);
        stats.reverse();
        for entry in stats.iter().take(10) {
            println!("{} => {}", entry.0, entry.1);
        }
        println!("Skipped {} files.", skipped);

        return Ok(());
    }
    return Ok(());
}
