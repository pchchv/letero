use std::net::Ipv4Addr;
use axum::{Router, routing::get};
use server::init_logs;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let _guard = init_logs();
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let port = 4000;
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, port))
        .await
        .expect("failed to bind port");

    tracing::info!("Listening on port {port}");
}