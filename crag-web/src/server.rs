use crate::handler;
use crate::handler::HandlerTrait;
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

type HandlerMap = HashMap<request::Request, handler::Handler>;
struct Handlers {
    valid_handlers: HandlerMap,
    error_handler: handler::Handler,
}
impl Handlers {
    fn handle_error(&self, req: Request) -> Result<Response> {
        self.error_handler.handle(req)
    }
}

pub struct Server {
    tcp_listener: TcpListener,
    pool: threadpool::ThreadPool,
    handlers: Arc<Handlers>,
}

pub struct ServerBuilder {
    handlers: HandlerMap,
    error_handler: Option<handler::Handler>,
}
impl ServerBuilder {
    /// Finalize the server builder and create a server instance.
    /// an error handler must always be defined or this will err.
    pub fn finalize(self, addr: impl ToSocketAddrs, pool_size: usize) -> Result<Server> {
        // Check to see that there is a handler for 404 errors
        let error_handler = match self.error_handler {
            Some(eh) => eh,
            None => anyhow::bail!("No handler for 404 errors"),
        };

        let socket_addr = addr
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| anyhow::anyhow!("Could not resolve address"))?;

        let tcp_listener = TcpListener::bind(socket_addr)?;
        let pool = threadpool::ThreadPool::build(pool_size)?;
        let handlers = Arc::new(Handlers {
            valid_handlers: self.handlers,
            error_handler,
        });

        let server = Server {
            tcp_listener,
            pool,
            handlers,
        };

        Ok(server)
    }
    pub fn register_handler(
        mut self,
        r: request::Request,
        handler: impl HandlerTrait + 'static + Send + Sync,
    ) -> Self {
        self.handlers.insert(r, Box::new(handler));
        self
    }

    pub fn register_error_handler(
        mut self,
        handler: impl HandlerTrait + 'static + Send + Sync,
    ) -> Result<Self> {
        if let Some(_) = self.error_handler {
            anyhow::bail!("Error handler already registered");
        }
        self.error_handler = Some(Box::new(handler));
        Ok(self)
    }
}

impl Server {
    pub fn build() -> ServerBuilder {
        ServerBuilder {
            handlers: HashMap::new(),
            error_handler: None,
        }
    }
    pub fn run(&self) -> Result<()> {
        for stream in self.tcp_listener.incoming() {
            let mut stream = stream?;
            let handlers = self.handlers.clone();

            self.pool.execute(move || {
                if let Err(e) = handle_connection(&handlers, &mut stream) {
                    // Error boundary for the thread handling the connection
                    error!("Error handling connection: {e:?}");
                    _ = stream.write_all("HTTP/1.1 500 Internal Server Error\r\n\r\n".as_bytes());
                }
            });
        }
        Ok(())
    }
}

fn handle_connection<S>(handlers: &Handlers, stream: &mut S) -> Result<()>
where
    S: Read + Write,
{
    let req = read_and_parse_request(stream)
        .map_err(|e| anyhow::anyhow!("Error parsing request: {e:?}"))?;

    // build response
    let response = match handlers.valid_handlers.get(&req) {
        Some(handler) => handler.handle(req),
        None => handlers.handle_error(req),
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
            if next_line.is_empty() || next_line == "\r" || next_line == "\r\n" {
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

fn parse_request<IT, S>(lines: IT) -> Result<(request::Request, usize)>
where
    IT: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut lines = lines.into_iter();

    // build request from header
    let first_line = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("No request line found"))?;
    let req = request::Request::parse(first_line)?;

    let content_length = match req {
        Request::GET(_) => 0,
        Request::POST(_, _) => {
            lines
                // .lines()
                .find(|line| line.as_ref().starts_with("Content-Length:"))
                .and_then(|line| {
                    line.as_ref()
                        .trim()
                        .split(':')
                        .nth(1)
                        .and_then(|value| value.trim().parse::<usize>().ok())
                })
                .unwrap_or(0)
        }
    };

    Ok((req, content_length))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::response::Response;

    #[test]
    fn test_builder_pattern() -> Result<()> {
        let _server = Server::build()
            .register_handler(request::Request::GET("/".to_owned()), |_req| {
                Ok(Response::Ok("Hello, Crag-Web!".to_string()))
            })
            .register_error_handler(handler::default_error_404_handler)?
            .finalize(("127.0.0.1", 23456), 4)
            .unwrap();

        Ok(())
    }

    #[test]
    fn test_no_error_handler_fails() -> Result<()> {
        let server = Server::build()
            .register_handler(request::Request::GET("/".to_owned()), |_req| {
                Ok(Response::Ok("Hello, Crag-Web!".to_string()))
            })
            .finalize(("127.0.0.1", 23458), 4);
        assert!(server.is_err());
        Ok(())
    }

    #[test]
    fn test_parse_request() -> Result<()> {
        let lines = &["GET / HTTP/1.1"];
        let res = parse_request(lines.iter());
        assert!(res.is_ok());
        Ok(())
    }

    #[test]
    fn test_parse_request_with_no_lines() -> Result<()> {
        // this is silly, we wouldn't use hash set but wanted to demonstrate
        // that any sort of iteratable container can be passed into parse_request.
        let empty_hash = HashSet::<&str>::new();
        let res = parse_request(empty_hash);
        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("No request line found"));
        Ok(())
    }
}
