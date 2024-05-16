pub enum Response {
    Ok(String),
    NotFound(String),
}
const HTML_TYPE: &str = "Content-Type: text/html";
impl From<Response> for Vec<u8> {
    fn from(value: Response) -> Vec<u8> {
        match value {
            Response::Ok(body) => {
                const STATUS_LINE: &str = "HTTP/1.0 200 OK";
                to_output(STATUS_LINE, HTML_TYPE, body.as_str())
            }
            Response::NotFound(_) => {
                const STATUS_LINE: &str = "HTTP/1.0 404 Not Found";
                const BODY: &str = include_str!("../static/html/404.html");
                to_output(STATUS_LINE, HTML_TYPE, BODY)
            }
        }
        .into_bytes()
    }
}

fn to_output(status: &str, content_type: &str, body: &str) -> String {
    format!(
        "{status}\r\nContent-Type: {type}\r\nContent-Length: {len}\r\n\r\n{body}",
        status = status,
        type = content_type,
        len = body.len(),
        body = body
    )
}
