use axum::{Router, routing::get};
use server::init_logs;

#[tokio::main]
async fn main() {
    let _guard = init_logs();

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
}