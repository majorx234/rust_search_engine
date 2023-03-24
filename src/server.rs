use crate::model::TermIndex;
use serde_json::{Result, Value};
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::str;
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

fn get_con_content_type(request: &Request) -> &str {
    for header in request.headers() {
        match header.field.as_str().as_str() {
            "Content-type" => return header.value.as_str(),
            _ => (),
        }
    }
    ""
}

fn serve_404(request: Request) -> io::Result<()> {
    request.respond(Response::from_string("404").with_status_code(StatusCode(404)))
}

fn serve_500(request: Request) -> io::Result<()> {
    request.respond(Response::from_string("500").with_status_code(StatusCode(500)))
}

fn serve_400(request: Request, message: &str) -> io::Result<()> {
    request.respond(
        Response::from_string(format!("400: {}", message)).with_status_code(StatusCode(400)),
    )
}

pub fn serve_static_file(
    request: Request,
    file_path: &PathBuf,
    content_type: &str,
) -> io::Result<()> {
    let content_type_header = Header::from_bytes("Content-Type", content_type).expect("Header ok");

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!(
                "ERROR: could not serve file {}: {}",
                file_path.display(),
                err
            );
            if err.kind() == io::ErrorKind::NotFound {
                return serve_404(request);
            }
            return serve_500(request);
        }
    };

    request.respond(Response::from_file(file).with_header(content_type_header))
}

pub fn serve_search(index: &TermIndex, mut request: Request) -> io::Result<()> {
    if get_con_content_type(&request) == "application/json" {
        let mut buf = String::new();
        if let Err(err) = request.as_reader().read_to_string(&mut buf) {
            eprintln!("ERROR: could not read the body of the request: {err}");
            return serve_500(request);
        }
        let body_json_data: Value = serde_json::from_str(&buf)?;
        if let Some(search_input) = body_json_data["search_input"].as_str() {
            let search_input_chars: Vec<char> = search_input.chars().collect();
            let result = match index.search_query(&search_input_chars) {
                Ok(result) => result,
                Err(()) => return serve_500(request),
            };

            let json = match serde_json::to_string(&result.iter().take(20).collect::<Vec<_>>()) {
                Ok(json) => json,
                Err(err) => {
                    eprintln!("ERROR: could not convert search results to JSON: {err}");
                    return serve_500(request);
                }
            };

            let content_type_header = Header::from_bytes("Content-Type", "application/json")
                .expect("That we didn't put any garbage in the headers");
            return request.respond(Response::from_string(&json).with_header(content_type_header));
        }
    }
    Ok(())
}
