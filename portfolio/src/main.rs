use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport};

use crag_web::request;
use crag_web::response;
use crag_web::server;
use std::net::ToSocketAddrs;

const STATIC_FILES: &str = "/Users/miguelalizo/projects/portfolio-rust-server/crag-web-rs/crag-web/examples/portfolio/static/";

// GET /not_found
fn not_found(_req: request::Request) -> response::Response {
    let filename = format!("{STATIC_FILES}html/404.html");
    let html = std::fs::read_to_string(filename).unwrap(); //?;;
                                                           // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let len = html.len();

    // format http response
    let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{html}");

    response::Response {
        content: response.as_bytes().to_vec(),
    }
}

// GET /index
fn index(_req: request::Request) -> response::Response {
    let filename = format!("{STATIC_FILES}html/index.html");
    let html = std::fs::read_to_string(filename).unwrap(); //?;;
                                                           // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{html}");

    response::Response {
        content: response.as_bytes().to_vec(),
    }
}

fn send_email() {
    // Set up SMTP credentials (your Gmail address and password)
    let email = "pinchonalizo@gmail.com";
    let password = "nibgtodnatvxcamx";
    let smtp_server = "smtp.gmail.com";

    // Create SMTP client with SSL
    let _smtp_client = SmtpTransport::starttls_relay(smtp_server)
        .unwrap()
        .credentials(Credentials::new(email.to_string(), password.to_string()))
        .build();

    // Define the email
    let _email = Message::builder()
        .from("miguel.e.alizo@gmail.com".parse().unwrap())
        .to("pinchonalizo@gmail.com".parse().unwrap())
        .subject("Rust Email")
        .body(String::from("Hello, this is a test email from Rust!"))
        .unwrap();

    // Send the email
    // match smtp_client.send(&email) {
    //     Ok(_) => println!("Email sent successfully!"),
    //     Err(e) => eprintln!("Failed to send email: {:?}", e),
    // }
}

// GET /contact
fn contact(req: request::Request) -> response::Response {
    let filename = match req {
        request::Request::POST(_, _body) => {
            // println!("{}", body);
            send_email();
            format!("{STATIC_FILES}html/thanks.html")
        }
        _ => format!("{STATIC_FILES}html/contact.html"),
    };
    let html = std::fs::read_to_string(filename).unwrap(); //?;;
                                                           // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{html}");

    response::Response {
        content: response.as_bytes().to_vec(),
    }
}

// GET /css/default.css
fn css_default(_req: request::Request) -> response::Response {
    let filename = format!("{STATIC_FILES}css/default.css");
    let css = std::fs::read(filename).unwrap(); //?;;
                                                // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = css.len();

    // format http response
    let response =
        format!("{status_line}\r\nContent-Type: text/css\r\nContent-Length: {len}\r\n\r\n",);
    let mut full_response = response.into_bytes();
    full_response.extend(css);

    response::Response {
        content: full_response,
    }
}

// GET /css/blue.css
fn css_blue(_req: request::Request) -> response::Response {
    let filename = format!("{STATIC_FILES}css/blue.css");
    let css = std::fs::read(filename).unwrap(); //?;;
                                                // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = css.len();

    // format http response
    let response =
        format!("{status_line}\r\nContent-Type: text/css\r\nContent-Length: {len}\r\n\r\n",);
    let mut full_response = response.into_bytes();
    full_response.extend(css);

    response::Response {
        content: full_response,
    }
}

// GET /css/green.css
fn css_green(_req: request::Request) -> response::Response {
    let filename = format!("{STATIC_FILES}css/green.css");
    let css = std::fs::read(filename).unwrap(); //?;;
                                                // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = css.len();

    // format http response
    let response =
        format!("{status_line}\r\nContent-Type: text/css\r\nContent-Length: {len}\r\n\r\n",);
    let mut full_response = response.into_bytes();
    full_response.extend(css);

    response::Response {
        content: full_response,
    }
}

// GET /css/purple.css
fn css_purple(_req: request::Request) -> response::Response {
    let filename = format!("{STATIC_FILES}css/purple.css");
    let css = std::fs::read(filename).unwrap(); //?;;
                                                // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = css.len();

    // format http response
    let response =
        format!("{status_line}\r\nContent-Type: text/css\r\nContent-Length: {len}\r\n\r\n",);
    let mut full_response = response.into_bytes();
    full_response.extend(css);

    response::Response {
        content: full_response,
    }
}

// GET /scripts/script.js
fn js_script(_req: request::Request) -> response::Response {
    let filename = format!("{STATIC_FILES}scripts/script.js");
    let html = std::fs::read(filename).unwrap(); //?;;
                                                 // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response = format!(
        "{status_line}\r\nContent-Type: application/javascript\r\nContent-Length: {len}\r\n\r\n"
    );
    let mut full_response = response.into_bytes();
    full_response.extend(html);

    response::Response {
        content: full_response,
    }
}

// GET /image/me.jpeg
fn image_me(_req: request::Request) -> response::Response {
    let filename = format!("{STATIC_FILES}images/me.jpeg");
    let html = std::fs::read(filename).unwrap(); //?;;
                                                 // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response =
        format!("{status_line}\r\nContent-Type: image/jpeg\r\nContent-Length: {len}\r\n\r\n",);
    let mut full_response = response.into_bytes();
    full_response.extend(html);

    response::Response {
        content: full_response,
    }
}

// GET /image/404.jpeg
fn image_404(_req: request::Request) -> response::Response {
    let filename = format!("{STATIC_FILES}images/404.jpeg");
    let html = std::fs::read(filename).unwrap(); //?;;
                                                 // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response =
        format!("{status_line}\r\nContent-Type: image/jpeg\r\nContent-Length: {len}\r\n\r\n",);
    let mut full_response = response.into_bytes();
    full_response.extend(html);

    response::Response {
        content: full_response,
    }
}

// GET /image/linkedin.jpeg
fn image_linkedin(_req: request::Request) -> response::Response {
    let filename = format!("{STATIC_FILES}images/linkedin.jpeg");
    let html = std::fs::read(filename).unwrap(); //?;;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response =
        format!("{status_line}\r\nContent-Type: image/jpeg\r\nContent-Length: {len}\r\n\r\n");

    let mut full_response = response.into_bytes();
    full_response.extend(html);

    response::Response {
        content: full_response,
    }
}

// GET /image/linkedin.jpeg
fn image_mail_sent(_req: request::Request) -> response::Response {
    let filename = format!("{STATIC_FILES}images/mail_sent.jpeg");
    let html = std::fs::read(filename).unwrap(); //?;;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response =
        format!("{status_line}\r\nContent-Type: image/jpeg\r\nContent-Length: {len}\r\n\r\n");

    let mut full_response = response.into_bytes();
    full_response.extend(html);

    response::Response {
        content: full_response,
    }
}

// GET <bad request>
fn error_404(_req: request::Request) -> response::Response {
    let filename = format!("{STATIC_FILES}html/404.html");
    let html = std::fs::read(filename).unwrap(); //?;;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response =
        format!("{status_line}\r\nContent-Type: text/html\r\nContent-Length: {len}\r\n\r\n");

    let mut full_response = response.into_bytes();
    full_response.extend(html);

    response::Response {
        content: full_response,
    }
}

fn main() -> std::io::Result<()> {
    // validate addr
    let addr = "127.0.0.1:8010";
    let socket_addr = match addr.to_socket_addrs() {
        Ok(addr_iter) => addr_iter,
        Err(_) => panic!("could not resolve socket address"),
    }
    .next()
    .unwrap();

    // Create server
    let pool_size = 4;
    let handlers = std::collections::HashMap::new();

    let srvr = server::Server::build(socket_addr, pool_size, handlers)
        .expect("Unable to create Server")
        .register_error_handler(error_404)
        .register_handler(
            request::Request::GET(String::from("/images/404.jpeg")),
            image_404,
        )
        .register_handler(request::Request::GET(String::from("/")), index)
        .register_handler(request::Request::GET(String::from("/contact")), contact)
        .register_handler(request::Request::GET(String::from("/not_found")), not_found)
        .register_handler(
            request::Request::GET(String::from("/scripts/script.js")),
            js_script,
        )
        .register_handler(
            request::Request::GET(String::from("/css/default.css")),
            css_default,
        )
        .register_handler(
            request::Request::GET(String::from("/css/blue.css")),
            css_blue,
        )
        .register_handler(
            request::Request::GET(String::from("/css/green.css")),
            css_green,
        )
        .register_handler(
            request::Request::GET(String::from("/css/purple.css")),
            css_purple,
        )
        .register_handler(
            request::Request::GET(String::from("/images/me.jpeg")),
            image_me,
        )
        .register_handler(
            request::Request::GET(String::from("/images/linkedin.jpeg")),
            image_linkedin,
        )
        .register_handler(
            request::Request::GET(String::from("/images/mail_sent.jpeg")),
            image_mail_sent,
        )
        .register_handler(
            request::Request::POST(String::from("/contact"), String::default()),
            contact,
        );
    // .add_handler(request::Request::POST(String::from("contact"), String::default()), post_contact);

    // run Server
    srvr.run();

    Ok(())
}
