use anyhow;
use std::net::SocketAddr;
use tide::http::mime::JSON;
use tide::prelude::*;
use tide::Request;
use trampoline::{check_email, CheckEmailInput, CheckEmailInputProxy};

#[derive(Debug, Deserialize)]
pub struct Proxy {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EmailCheckRequest {
    to_email: String,
    from_email: Option<String>,
    hello_name: Option<String>,
    proxy: Option<Proxy>,
    smtp_port: Option<u16>,
}

pub async fn handle_check_email(mut req: Request<()>) -> tide::Result {
    let body_str = req.body_string().await?;
    let params: EmailCheckRequest = serde_json::from_str(&body_str)?;
    let mut input = CheckEmailInput::new(params.to_email);
    input
        .set_from_email(params.from_email.unwrap_or("hello@example.com".to_string()))
        .set_hello_name(params.hello_name.unwrap_or("example.com".to_string()))
        .set_smtp_port(params.smtp_port.unwrap_or(25));

    if let Some(proxy) = params.proxy {
        input.set_proxy(CheckEmailInputProxy {
            host: proxy.host,
            port: proxy.port,
            username: proxy.username,
            password: proxy.password,
        });
    }
    // Create a Tokio runtime
    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();

    let result = tokio_runtime.block_on(check_email(&input));
    println!("result: {:?}", result);
    match serde_json::to_string_pretty(&result) {
        Ok(output) => {
            // Using JSON as the output response type and setting HTTP status as 200
            Ok(tide::Response::builder(200)
                .body(tide::Body::from_string(output))
                .content_type(JSON)
                .build())
        }
        Err(_) => {
            // Generating a 500 error if serializing JSON fails
            Err(tide::Error::new(
                500,
                anyhow::anyhow!("Failed to serialize JSON"),
            ))
        }
    }
}

pub async fn run<A: Into<SocketAddr> + tide::listener::ToListener<()>>(
    addr: A,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut app = tide::new();
    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.at("/").post(handle_check_email);
    app.listen(addr).await?;
    Ok(())
}
