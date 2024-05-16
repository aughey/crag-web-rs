#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Request {
    GET(String),
    POST(String, String),
}

impl Request {
    // should this be from implementation instead?
    pub fn parse(request_line: impl AsRef<str>) -> anyhow::Result<Request> {
        let request_line = request_line.as_ref();
        println!("{request_line}");
        let mut parts = request_line.split_whitespace();

        let method = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("No method found"))?;
        let uri = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("No URI found"))?;
        let protocol = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("No protocol found"))?;

        if protocol != "HTTP/1.1" {
            anyhow::bail!("Server can only work with HTTP/1.1");
        }

        // should have no more parts left
        if parts.next().is_some() {
            anyhow::bail!("Invalid request line: extra values after parts");
        }

        let ret = match method {
            "GET" => Request::GET(String::from(uri)),
            "POST" => Request::POST(String::from(uri), String::default()),
            _ => anyhow::bail!("Invalid method {method}"),
        };
        Ok(ret)
    }
    pub fn add_body(&mut self, body: String) {
        if let Request::POST(_, ref mut b) = self {
            *b = body;
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_parser_happy_path() {
        let req = Request::parse(&String::from("GET / HTTP/1.1")).unwrap();
        assert_eq!(req, Request::GET(String::from("/")));

        let req = Request::parse(&String::from("POST / HTTP/1.1")).unwrap();
        assert_eq!(req, Request::POST(String::from("/"), String::default()));
    }

    #[test]
    fn test_no_verb_found() {
        let req = Request::parse(&String::from(""));
        assert!(req.is_err(), "Returned request is: {req:?}");
        assert!(req.err().unwrap().to_string().contains("No method found"));
    }

    #[test]
    fn test_request_parser_bad_verbs() {
        let req = Request::parse(&String::from("FOO / HTTP/1.1"));
        assert!(req.is_err(), "Returned request is: {req:?}");
    }
    #[test]
    fn test_good_paths() {
        let req = Request::parse(&String::from("GET /foo/bar HTTP/1.1")).unwrap();
        assert_eq!(req, Request::GET(String::from("/foo/bar")));
    }
    #[test]
    fn test_bad_path() {
        let req = Request::parse(&String::from("GET"));
        assert!(req.is_err(), "Returned request is: {req:?}");
        assert!(req.err().unwrap().to_string().contains("No URI found"));
    }

    #[test]
    fn test_missing_protocol() {
        let req = Request::parse(&String::from("GET /"));
        assert!(req.is_err(), "Returned request is: {req:?}");
        assert!(req.err().unwrap().to_string().contains("No protocol found"));
    }

    #[test]
    fn test_bad_protocol_name() {
        let req = Request::parse(&String::from("GET / HTTP/1.0"));
        assert!(req.is_err(), "Returned request is: {req:?}");
        assert!(req
            .err()
            .unwrap()
            .to_string()
            .contains("Server can only work with HTTP/1.1"));
    }
}
