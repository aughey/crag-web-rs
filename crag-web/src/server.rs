use crate::handler;
use crate::request;
use crate::threadpool;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

#[derive(Debug)]
pub enum ServerError {
    ServerCreation(std::io::Error),
    PoolSizeError(threadpool::PoolCreationError),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for ServerError {}

#[derive(Debug)]
pub struct Server {
    tcp_listener: TcpListener,
    pool: threadpool::ThreadPool,
    handlers: HashMap<request::Request, handler::Handler>,
}

impl Server {
    pub fn build(
        socket_addr: SocketAddr,
        pool_size: usize,
        handlers: HashMap<request::Request, handler::Handler>,
    ) -> Result<Server, ServerError> {
        let tcp_listener = TcpListener::bind(socket_addr).map_err(ServerError::ServerCreation)?;

        let pool = threadpool::ThreadPool::build(pool_size).map_err(ServerError::PoolSizeError)?;

        let server = Server {
            tcp_listener,
            pool,
            handlers,
        };

        Ok(server)
    }

    pub fn register_handler(mut self, r: request::Request, handler: handler::Handler) -> Self {
        self.handlers.insert(r, handler);
        self
    }

    pub fn register_error_handler(self, handler: handler::Handler) -> Self {
        let request = request::Request::UNIDENTIFIED;
        self.register_handler(request, handler)
    }

    pub fn run(&self) {
        for stream in self.tcp_listener.incoming() {
            match stream {
                Ok(stream) => {
                    // TODO: use Arc instead of cloning entire hashmap
                    let cloned_handlers = self.handlers.clone();

                    self.pool.execute(|| {
                        handle_connection(cloned_handlers, stream); //?
                    });
                }
                Err(e) => panic!("{} Error handling connection!", e),
            }
        }
    }
}

fn handle_connection(handlers: HashMap<request::Request, handler::Handler>, mut stream: TcpStream) {
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
    stream.write_all(&response.content).unwrap(); //?;
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
