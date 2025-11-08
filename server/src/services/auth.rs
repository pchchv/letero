use axum::response::Response;
use crate::{
    models::users::User,
    services::trace::TraceId,
    error::ApiError,
};

pub const SESSION_COOKIE_NAME: &str = "session";

pub struct Auth {
    pub session: String,
    pub user: User,
}

fn unauthorized(trace_id: TraceId) -> Response {
    ApiError::Unauthorized { trace_id }.into_response()
}