use anyhow::Result;
use crag_web::{handler, request, response, server::Server};

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::build()
        .register_handler(request::Request::GET(String::from("/hello")), hello_handler)
        .register_error_handler(handler::default_error_404_handler)
        .finalize(("127.0.0.1", 12345), 4)?;

    server.run();

    Ok(())
}

fn hello_handler(_req: request::Request) -> response::Response {
    response::Response::Ok("Hello world".to_string())
}
