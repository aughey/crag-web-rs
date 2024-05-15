use std::thread;

use anyhow::Result;
use crag_web::{handler, request, response, server::Server};

#[tokio::test]
async fn test_index() -> Result<()> {
    let server = Server::build()
        .register_handler(request::Request::GET(String::from("/hello")), hello_handler)
        .register_handler(request::Request::GET(String::from("/error")), |_| {
            Err(anyhow::anyhow!("error"))
        })
        .register_handler(request::Request::GET(String::from("/foo")), |_| {
            Ok(response::Response::Ok("foo".to_string()))
        })
        .register_error_handler(handler::default_error_404_handler)
        .finalize(("127.0.0.1", 12345), 4)?;

    let _server_join = thread::spawn(move || {
        server.run().unwrap();
    });

    let r = reqwest::get("http://127.0.0.1:12345/bad").await?;
    assert!(r.status().is_client_error());

    let r = reqwest::get("http://127.0.0.1:12345/hello").await?;
    assert!(r.status().is_success());
    assert_eq!(r.text().await?, "Hello, Crag-Web!");

    let r = reqwest::get("http://127.0.0.1:12345/foo").await?;
    assert!(r.status().is_success());
    assert_eq!(r.text().await?, "foo");

    let r = reqwest::get("http://127.0.0.1:12345/error").await?;
    assert!(r.status().is_server_error());

    Ok(())
}

fn hello_handler(_req: request::Request) -> anyhow::Result<response::Response> {
    Ok(response::Response::Ok("Hello, Crag-Web!".to_string()))
}
