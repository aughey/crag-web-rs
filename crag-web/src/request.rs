#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Request {
    GET(String),
    POST(String, String),
    UNIDENTIFIED,
}

impl Request {
    // should this be from implementation instead?
    pub fn build(request_line: String) -> Request {
        println!("{request_line}");
        let mut parts = request_line.split_whitespace();

        let method = parts.next().unwrap_or("GET");
        let uri = parts.next().unwrap_or("not_implemented");
        let protocol = parts.next().unwrap_or("HTTP/1.1");

        if protocol != "HTTP/1.1" {
            panic!("Server can only work with HTTP/1.1");
        }

        match method {
            "GET" => Request::GET(String::from(uri)),
            "POST" => Request::POST(String::from(uri), String::default()),
            _ => Request::UNIDENTIFIED,
        }
    }
    pub fn add_body(&mut self, body: String) {
        if let Request::POST(_, ref mut b) = self {
            *b = body;
        };
    }
}
