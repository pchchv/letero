use std::sync::Arc;
use axum::{
    body::Body,
    http::Request,
    extract::State,
    middleware::Next,
    response::{Response, IntoResponse},
};
use crate::{
    AppState,
    error::ApiError,
    models::users::User,
    services::trace::TraceId,
};

pub const SESSION_COOKIE_NAME: &str = "session";

pub struct Auth {
    pub session: String,
    pub user: User,
}

fn unauthorized(trace_id: TraceId) -> Response {
    ApiError::Unauthorized { trace_id }.into_response()
}

pub async fn auth(
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    tracing::info!("authenticating user...");
    let Some(trace_id) = req.extensions().get::<TraceId>() else {
        tracing::warn!("trace id not found");
        return ApiError::Internal.into_response();
    };

    tracing::trace!("parsing cookie...");
    let cookie = req
        .headers()
        .get_all("Cookie")
        .into_iter()
        .filter_map(|c| c.to_str().ok())
        .filter_map(|c| c.parse::<cookie::Cookie>().ok())
        .find(|c| c.name() == SESSION_COOKIE_NAME);

    tracing::trace!("getting session uid...");
    let session_uid = match cookie {
        Some(c) => c.value().to_owned(),
        None => {
            tracing::warn!("session cookie not found");
            return unauthorized(trace_id.clone());
        }
    };

    let user = match state.users.get_user_by_session(&session_uid).await {
        Ok(user) => user,
        Err(err) => {
            tracing::error!("failed to get user: {}", err);
            return unauthorized(trace_id.clone());
        }
    };

    let auth_state = Arc::new(Auth {
        session: session_uid.to_owned(),
        user,
    });

    req.extensions_mut().insert(auth_state);
    next.run(req).await
}