use crate::request::Request;
use crate::response;

pub type Handler = fn(Request) -> response::Response;

/// Default handler for 404 errors
pub fn default_error_404_handler(_request: Request) -> response::Response {
    let bytes = include_bytes!("../static/html/404.html");
    let status_line = "HTTP/1.1 404 Not Found";
    let len = bytes.len();

    // format http response
    let response =
        format!("{status_line}\r\nContent-Type: text/html\r\nContent-Length: {len}\r\n\r\n");

    let mut full_response = response.into_bytes();
    full_response.extend(bytes);

    response::Response {
        content: full_response,
    }
}
