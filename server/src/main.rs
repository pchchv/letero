use std::{net::Ipv4Addr, sync::Arc};
use axum::{Router, middleware, routing::get};
use server::{init_logs, init_db, services::trace::trace};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let _guard = init_logs();
    let db = init_db().await;
    let users = Arc::new(PgUsersRepository::new(db));
    let sessions = Arc::new(PgSessionsRepository::new(db.clone()));
    let state = Arc::new(AppState { users, sessions });
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(state)
        .layer(middleware::from_fn(trace));

    let port = 4000;
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, port))
        .await
        .expect("failed to bind port");

    tracing::info!("Listening on port {port}");

    axum::serve(listener, app)
        .await
        .expect("failed to start server");
}