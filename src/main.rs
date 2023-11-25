use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use hyper::server::conn::http1;
use hyper::service::service_fn;
use tokio::net::TcpListener;

#[path = "./utils/mod.rs"]
mod utils;
use utils::ServerConfig;

#[path = "./lib/tokio/mod.rs"]
mod support;
use support::TokioIo;

#[path = "./services/mod.rs"]
mod services;
use services::echo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr_http = SocketAddr::from((
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        ServerConfig::port_from_env(),
    ));
    let listener_http = TcpListener::bind(addr_http).await?;
    let pretty_addr = format!("{}", addr_http).replace("0.0.0.0", "localhost");

    println!("HTTP on http://{}", pretty_addr);

    loop {
        let (stream_http, _) = listener_http.accept().await?;
        let io_http = TokioIo::new(stream_http);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io_http, service_fn(echo))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
