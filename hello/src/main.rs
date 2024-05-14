use crag_web::response;
use std::net::ToSocketAddrs;

// get "/hello"
#[allow(dead_code)]
fn hello_handler() -> response::Response {
    let body = "Hello, Crag-Web!";
    let status_line = "HTTP/1.1 200 OK";
    let len = body.len();

    // format http response
    let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{body}");
    response::Response {
        content: response.as_bytes().to_vec(),
    }
}

// get <bad request>
#[allow(dead_code)]
fn error_404_handler() -> response::Response {
    let body = "404 not found";
    let status_line = "HTTP/1.1 404 Not Found";
    let len = body.len();

    // format http response
    let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{body}");
    response::Response {
        content: response.as_bytes().to_vec(),
    }
}

fn main() -> std::io::Result<()> {
    // validate addr
    let addr = "127.0.0.1:8010";
    let _socket_addr = match addr.to_socket_addrs() {
        Ok(addr_iter) => addr_iter,
        Err(_) => panic!("could not resolve socket address"),
    }
    .next()
    .unwrap();

    // Create server
    let _pool_size = 4;
    //    let handlers = std::collections::HashMap::new();
    // let app = server::Server::build(socket_addr, pool_size, handlers)
    //     .expect("Unable to create Server")
    //     .register_error_handler(error_404_handler)
    //     .register_handler(request::Request::GET(String::from("/hello")), hello_handler);

    // // Run server
    // app.run();

    Ok(())
}
