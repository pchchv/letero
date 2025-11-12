use std::{net::Ipv4Addr, sync::Arc};
use axum::middleware;
use server::{
    docs::ApiDoc,
    init_db, init_logs,
    AppState,
    rand::SmallRandom,
    services::{session, trace::trace},
    controllers::{chats, events, messages, search,users::{self}},
};
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::services::{ServeDir, ServeFile};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

#[tokio::main]
async fn main() {
    let _guard = init_logs();
    let db = init_db().await;
    session::start_cleanup_task(db.clone());
    let rng = Arc::new(Mutex::new(SmallRandom::new(807234275934919497)));
    let state = Arc::new(AppState::new(rng, db));
    let root_path = std::env::current_exe().expect("failed to get executable path");
    let root_path = root_path.parent().expect("failed to get parent directory");
    let (app, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(events::events))
        .routes(routes!(messages::new_message, messages::get_messages))
        .routes(routes!(chats::remove_chat))
        .routes(routes!(chats::new_chat, chats::get_chats))
        .routes(routes!(users::get_user))
        .routes(routes!(users::logout_user))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            server::services::auth::auth,
        ))
        .routes(routes!(search::search_users))
        .routes(routes!(users::login_user))
        .routes(routes!(users::new_user))
        .with_state(state)
        .layer(middleware::from_fn(trace))
        .nest_service(
            "/assets",
            ServeDir::new(root_path.join("public").join("assets")),
        )
        .fallback_service(ServeFile::new(root_path.join("public").join("index.html")))
        .split_for_parts();

    let app = app.merge(utoipa_rapidoc::RapiDoc::with_url(
        "/docs",
        "/openapi.json",
        api,
    ));

    let port = 4000;
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port))
        .await
        .expect("failed to bind port");

    tracing::info!("Listening on port {port}");

    axum::serve(listener, app)
        .await
        .expect("failed to start server");
}