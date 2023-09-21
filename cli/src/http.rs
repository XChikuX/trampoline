use tide::prelude::*;
use tide::Request;
use trampoline::{check_email, CheckEmailInput, CheckEmailInputProxy};

#[derive(Debug, Deserialize)]
pub struct EmailCheckRequest {
    to_email: String,
    from_email: Option<String>,
    hello_name: Option<String>,
    proxy: Option<Proxy>,
    smtp_port: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct Proxy {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
}

pub async fn handle_check_email(mut req: Request<()>) -> tide::Result {
    let params: EmailCheckRequest = req.body_json().await?;
    let mut input = CheckEmailInput::new(params.to_email);
    input
        .set_from_email(params.from_email.unwrap_or_else(|| CONF.from_email.clone()))
        .set_hello_name(params.hello_name.unwrap_or_else(|| CONF.hello_name.clone()))
        .set_smtp_port(params.smtp_port.unwrap_or(CONF.smtp_port));

    if let Some(proxy) = params.proxy {
        input.set_proxy(CheckEmailInputProxy {
            host: proxy.host,
            port: proxy.port,
            username: proxy.username,
            password: proxy.password,
        });
    }

    let result = check_email(&input).await;
    Ok(tide::Response::new(200).body_json(&result)?)
}

pub async fn run<A: Into<SocketAddr>>(
    addr: A,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut app = tide::new();
    app.at("/").post(handle_check_email);
    app.listen(addr).await?;
    Ok(())
}