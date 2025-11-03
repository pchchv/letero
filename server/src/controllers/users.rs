use crate::{AppState, models::users::NewUserRequest, services::trace::TraceId, error::ApiError};
use axum::{Form, Extension, extract::State, http::StatusCode};
use std::{collections::HashMap, sync::Arc};

pub async fn new_user(
    Extension(trace_id): Extension<TraceId>,
    State(state): State<Arc<AppState>>,
    Form(user): Form<NewUserRequest>,
) -> Result<(StatusCode, [(&'static str, String); 1]), ApiError> {
    tracing::trace!("validationg user credentials");
    let errors = validate_user(&user);

    if !errors.is_empty() {
        tracing::trace!("invalid user credentials, returning error");
        return Err(ApiError::Validation {
            fields: errors,
            trace_id,
        });
    }

    todo!()
}

fn validate_user(user: &NewUserRequest) -> HashMap<String, Vec<String>> {
    let mut errors = HashMap::new();
    let username_errors = user.username.validate();
    if !username_errors.is_empty() {
        tracing::trace!("invalid username: {}", username_errors.join(", "));
        errors.insert("username".to_owned(), username_errors);
    }

    let password_errors = user.password.validate();
    if !password_errors.is_empty() {
        tracing::trace!("invalid password: {}", password_errors.join(", "));
        errors.insert("password".to_owned(), password_errors);
    }

    errors
}