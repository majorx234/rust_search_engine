use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::str;
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

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
