use clap::{Arg, Command, Parser};
use search_engine;
use search_engine::model::{TermFreq, TermFreqPerDoc, TermIndex};
use search_engine::server::{serve_search, serve_static_file};
use serde_json;
use std::env::args;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::str::{from_utf8, FromStr};
use tiny_http::{self, Method, Response, Server};

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

    let tf_index: TermIndex = serde_json::from_reader(json_file)?;
    println!(
        "{} contains {} files {} terms",
        json_file_path.to_str().unwrap(),
        tf_index.term_freq_per_doc.len(),
        tf_index.doc_freq.len()
    );

    // tiny http server
    let server = Server::http("0.0.0.0:8000").unwrap();
    for request in server.incoming_requests() {
        println!(
            "received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );

        // register endpoints
        match (request.method(), request.url()) {
            (Method::Get, "/") | (Method::Get, "/index.html") => {
                println!("get index.html");
                let index_html = PathBuf::from_str("index.html").unwrap();
                serve_static_file(request, &index_html, "text/html; charset=utf-8");
            }
            (Method::Get, "/index.html") | (Method::Get, "/index.html") => {
                println!("get index.html");
                let index_html = PathBuf::from_str("index.html").unwrap();
                serve_static_file(request, &index_html, "text/html; charset=utf-8");
            }
            (Method::Get, "/index.js") => {
                println!("get index.js");
                let index_js = PathBuf::from_str("index.js").unwrap();
                serve_static_file(request, &index_js, "text/javascript; charset=utf-8");
            }
            (Method::Get, "/get_test") => {
                println!("get test");
            }
            (Method::Post, "/api/search") => {
                serve_search(&tf_index, request);
                println!("post");
            }
            _ => {
                println!("else {} {}", request.method(), request.url());
            }
        }
    }
    Ok(())
}
