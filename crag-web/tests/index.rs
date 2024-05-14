use std::thread;

use anyhow::Result;
use crag_web::{handler, request, response, server::Server};

#[tokio::test]
async fn test_index() -> Result<()> {
    let server = Server::build()
        .register_handler(request::Request::GET(String::from("/hello")), hello_handler)
        .register_error_handler(handler::default_error_404_handler)
        .finalize(("127.0.0.1", 12345), 4)?;

    // let server_join = tokio::task::spawn_blocking(move || {
    //     server.run();
    // });
    let _server_join = thread::spawn(move || {
        server.run();
    });

    let r = reqwest::get("http://127.0.0.1:12345/bad").await?;
    assert!(r.status().is_client_error());

    let r = reqwest::get("http://127.0.0.1:12345/hello").await?;
    assert!(r.status().is_success());

    assert_eq!(r.text().await?, "Hello, Crag-Web!");

    Ok(())
}

fn hello_handler(_req: request::Request) -> response::Response {
    let body = "Hello, Crag-Web!";
    let status_line = "HTTP/1.1 200 OK";
    let len = body.len();

    // format http response
    let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{body}");
    response::Response {
        content: response.as_bytes().to_vec(),
    }
}
