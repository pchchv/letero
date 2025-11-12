use std::{net::Ipv4Addr, sync::Arc};
use axum::{Router, middleware, routing::{get, post}};
use server::{
    init_db, init_logs,
    AppState,
    rand::SmallRandom,
    services::{session, trace::trace},
    controllers::users::{login_user, new_user},
};
use tokio::{net::TcpListener, sync::Mutex};

#[tokio::main]
async fn main() {
    let _guard = init_logs();
    let db = init_db().await;
    session::start_cleanup_task(db.clone());
    let rng = Arc::new(Mutex::new(SmallRandom::new(807234275934919497)));
    let state = Arc::new(AppState::new(rng, db));
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users", post(new_user))
        .route("/login", post(login_user))
        .with_state(state)
        .route_layer(middleware::from_fn(trace));

    let port = 4000;
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port))
        .await
        .expect("failed to bind port");

    tracing::info!("Listening on port {port}");

    axum::serve(listener, app)
        .await
        .expect("failed to start server");
}