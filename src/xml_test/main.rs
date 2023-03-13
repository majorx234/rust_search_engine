use clap::{Arg, Command, Parser};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::exit;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn chop(&mut self, n: usize) -> &'a [char] {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> &'a [char]
    where
        P: FnMut(&char) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }
        return self.chop(n);
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }
        // check numerics:
        if self.content[0].is_numeric() {
            return Some(self.chop_while(|x| x.is_numeric()));
        }
        // check keywords
        if self.content[0].is_alphabetic() {
            return Some(self.chop_while(|x| x.is_alphabetic()));
        }

        // other tokens
        return Some(self.chop(1));
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn index_document(content: &str) -> HashMap<String, u32> {
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
            content.push(' ');
        }
    }
    Ok(content)
}

fn main() -> io::Result<()> {
    let all_documents = HashMap::<PathBuf, HashMap<String, u32>>::new();
    let args = Args::parse();
    let xml_dir_path = PathBuf::from(args.xml_file_path);
    let xml_files = fs::read_dir(xml_dir_path)?;

    for xml_file_path in xml_files {
        let xml_file_path = xml_file_path?.path();
        let mut tf = HashMap::<String, u32>::new();
        if let Ok(content) = get_content_of_xml(&xml_file_path) {
            let char_content = content.chars().collect::<Vec<_>>();
            // println!("{}", content);
            for token in Lexer::new(&char_content) {
                println!(
                    "{:?}",
                    token
                        .iter()
                        .map(|x| x.to_ascii_uppercase())
                        .collect::<String>()
                );
            }
        }
    }
    /*
    for xml_file_path in xml_files {
        let xml_file_path = xml_file_path?.path();
        println!("}file_path: {}", xml_file_path.display());
        if let Ok(content) = get_content_of_xml(&xml_file_path) {
            println!("{}", content);
        }
    }*/
    Ok(())
}
