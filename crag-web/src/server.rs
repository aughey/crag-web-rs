use crate::handler;
use crate::request;
use crate::request::Request;
use crate::response::Response;
use crate::threadpool;
use anyhow::Result;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tracing::error;

pub struct Server {
    tcp_listener: TcpListener,
    pool: Option<threadpool::ThreadPool>,
    handlers: Arc<HashMap<request::Request, handler::Handler>>,
}

pub struct ServerBuilder {
    handlers: HashMap<request::Request, handler::Handler>,
}
impl ServerBuilder {
    pub fn finalize(self, addr: impl ToSocketAddrs, pool_size: usize) -> Result<Server> {
        let socket_addr = addr
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| anyhow::anyhow!("Could not resolve address"))?;

        let tcp_listener = TcpListener::bind(socket_addr)?;
        //        let pool = threadpool::ThreadPool::build(pool_size)?;

        let server = Server {
            tcp_listener,
            pool: None,
            handlers: Arc::new(self.handlers),
        };

        Ok(server)
    }
    pub fn register_handler(
        mut self,
        r: request::Request,
        handler: impl Fn(Request) -> anyhow::Result<Response> + 'static + Send + Sync,
    ) -> Self {
        self.handlers.insert(r, Box::new(handler));
        self
    }

    pub fn register_error_handler(
        self,
        handler: impl Fn(Request) -> anyhow::Result<Response> + 'static + Send + Sync,
    ) -> Self {
        let request = request::Request::UNIDENTIFIED;
        self.register_handler(request, Box::new(handler))
    }
}

impl Server {
    pub fn build() -> ServerBuilder {
        ServerBuilder {
            handlers: HashMap::new(),
        }
    }
    pub fn run(&self) -> Result<()> {
        for stream in self.tcp_listener.incoming() {
            let mut stream = stream?;
            let handlers = self.handlers.clone();

            //            self.pool.execute(move || {
            if let Err(e) = handle_connection(&handlers, &mut stream) {
                // Error boundary for the thread handling the connection
                error!("Error handling connection: {e:?}");
                _ = stream.write_all("HTTP/1.1 500 Internal Server Error\r\n\r\n".as_bytes());
            }
            //           });
        }
        Ok(())
    }
}

fn handle_connection<S>(
    handlers: &HashMap<request::Request, handler::Handler>,
    stream: &mut S,
) -> Result<()>
where
    S: Read + Write,
{
    let req = read_and_parse_request(stream)
        .map_err(|e| anyhow::anyhow!("Error parsing request: {e:?}"))?;
    let hashed_req = match req {
        request::Request::GET(ref a) => request::Request::GET(a.clone()),
        request::Request::POST(ref a, _) => request::Request::POST(a.clone(), String::default()),
        request::Request::UNIDENTIFIED => request::Request::UNIDENTIFIED,
    };

    // build response
    let response = match handlers.get(&hashed_req) {
        Some(handler) => handler(req),
        None => {
            // TODO: Figure out better way to handle 404 not found
            match handlers.get(&request::Request::UNIDENTIFIED) {
                Some(handler) => handler(req),
                None => handler::default_error_404_handler(req),
            }
        }
    };

    let response = response?;

    // write response into TcpStream
    stream.write_all(&Vec::<u8>::from(response))?;

    Ok(())
}

fn read_and_parse_request(stream: &mut impl Read) -> Result<request::Request> {
    // create buffer
    let mut buffer = BufReader::new(stream);

    // Read the HTTP request headers until end of header
    let lines = {
        let mut lines: Vec<String> = vec![];
        loop {
            let mut next_line = String::new();
            buffer.read_line(&mut next_line)?;
            if next_line == "" || next_line == "\r" || next_line == "\r\n" {
                break lines;
            }
            lines.push(next_line);
        }
    };

    let (req, _content_length) = parse_request(&lines)?;

    // Parse the request body based on Content-Length
    // let mut body_buffer = vec![];
    // buffer.read_to_end(&mut body_buffer)?;

    Ok(req)
}

fn parse_request(lines: &[String]) -> Result<(request::Request, usize)> {
    // build request from header
    let req = request::Request::build(lines.first().unwrap_or(&"/".to_owned()).to_owned());

    if let request::Request::POST(_, _) = req {
        // Find the Content-Length header
        let content_length = lines
            .iter()
            // .lines()
            .find(|line| line.starts_with("Content-Length:"))
            .and_then(|line| {
                line.trim()
                    .split(':')
                    .nth(1)
                    .and_then(|value| value.trim().parse::<usize>().ok())
            })
            .unwrap_or(0);
        panic!(
            "Need to read the body according to the content length, but we're not doing that yet",
        );
    };

    Ok((req, 0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::Response;

    #[test]
    fn test_builder_pattern() {
        let _server = Server::build()
            .register_handler(request::Request::GET("/".to_owned()), |_req| {
                Ok(Response::Ok("Hello, Crag-Web!".to_string()))
            })
            .register_error_handler(handler::default_error_404_handler)
            .finalize(("127.0.0.1", 23456), 4)
            .unwrap();
    }
}
