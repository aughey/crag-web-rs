use crate::handler;
use crate::request;
use crate::request::Request;
use crate::response::Response;
use crate::threadpool;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::ToSocketAddrs;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

#[derive(Debug)]
pub enum ServerError {
    BadSockAddr,
    ServerCreation(std::io::Error),
    PoolSizeError(threadpool::PoolCreationError),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for ServerError {}

pub struct Server {
    tcp_listener: TcpListener,
    pool: threadpool::ThreadPool,
    handlers: Arc<HashMap<request::Request, handler::Handler>>,
}

pub struct ServerBuilder {
    handlers: HashMap<request::Request, handler::Handler>,
}
impl ServerBuilder {
    pub fn finalize(
        self,
        addr: impl ToSocketAddrs,
        pool_size: usize,
    ) -> Result<Server, ServerError> {
        let socket_addr = match addr.to_socket_addrs() {
            Ok(addr_iter) => addr_iter,
            Err(_) => panic!("could not resolve socket address"),
        }
        .next()
        .ok_or(ServerError::BadSockAddr)?;

        let tcp_listener = TcpListener::bind(socket_addr).map_err(ServerError::ServerCreation)?;
        let pool = threadpool::ThreadPool::build(pool_size).map_err(ServerError::PoolSizeError)?;

        let server = Server {
            tcp_listener,
            pool: pool,
            handlers: Arc::new(self.handlers),
        };

        Ok(server)
    }
    pub fn register_handler(
        mut self,
        r: request::Request,
        handler: impl Fn(Request) -> Response + 'static + Send + Sync,
    ) -> Self {
        self.handlers.insert(r, Box::new(handler));
        self
    }

    pub fn register_error_handler(
        self,
        handler: impl Fn(Request) -> Response + 'static + Send + Sync,
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
    pub fn run(&self) {
        for stream in self.tcp_listener.incoming() {
            match stream {
                Ok(stream) => {
                    let handlers = self.handlers.clone();

                    self.pool.execute(move || {
                        handle_connection(handlers, stream); //?
                    });
                }
                Err(e) => panic!("{} Error handling connection!", e),
            }
        }
    }
}

fn handle_connection(
    handlers: Arc<HashMap<request::Request, handler::Handler>>,
    mut stream: TcpStream,
) {
    let req = parse_request(&mut stream).expect("Error parsing request");
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

    // write response into TcpStream
    stream.write_all(&Vec::<u8>::from(response)).unwrap(); //?;

    stream.shutdown(std::net::Shutdown::Both).unwrap(); //?; // close the stream (both directions

    println!("Wrote all");
}

// TODO: Fix return type
fn parse_request(stream: &mut TcpStream) -> Result<request::Request, std::io::Error> {
    // create buffer
    let mut request: Vec<String> = vec![];
    let mut buffer = BufReader::new(stream);

    // Read the HTTP request headers until end of header
    while request.is_empty() || request.last().insert(&String::default()).len() > 2 {
        let mut next_line = String::new();
        buffer.read_line(&mut next_line)?;
        request.push(next_line);
    }

    // build request from header
    let mut req = request::Request::build(request.first().unwrap_or(&"/".to_owned()).to_owned());

    if let request::Request::POST(_, _) = req {
        // Find the Content-Length header
        let content_length = request
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

        // Parse the request body based on Content-Length
        let mut body_buffer = vec![0; content_length];
        buffer.read_to_end(&mut body_buffer)?;

        // Add body to request
        req.add_body(String::from_utf8(body_buffer.clone()).unwrap_or_default());
    };

    Ok(req)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::Response;

    #[test]
    fn test_builder_pattern() {
        let _server = Server::build()
            .register_handler(request::Request::GET("/".to_owned()), |_req| {
                Response::Ok("Hello, Crag-Web!".to_string())
            })
            .register_error_handler(handler::default_error_404_handler)
            .finalize(("127.0.0.1", 23456), 4)
            .unwrap();
    }
}
