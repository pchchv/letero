use crate::{
    error::{ApiError, RepositoryError},
    models::users::{
        LoginUserRequest, PasswordHash, UserId,
    },
    rand::RandomGenerator,
    repositories::users::UsersRepository,
    services::trace::TraceId,
    state::AppState,
};
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

    let salt = state.random.lock().await.get_salt();
    let password = PasswordHash::new(&user.password, &salt);
    let user_id = create_user(&state.random, &*state.users, &user.username, &trace_id).await?;

    todo!()
}

fn validate_user(user: &LoginUserRequest) -> HashMap<String, Vec<String>> {
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

async fn create_user(
    rand: &tokio::sync::Mutex<dyn RandomGenerator>,
    users: &dyn UsersRepository,
    user: &LoginUserRequest,
    trace_id: &TraceId,
) -> Result<UserId, ApiError> {
    tracing::trace!("hashing password...");

    let salt = rand.lock().await.get_salt();
    let password_hash = PasswordHash::new(&user.password, &salt);

    tracing::trace!("saving user credintials in database...");
    let result = users.create_user(&user.username, password_hash).await;

    match result {
        Ok(id) => {
            tracing::info!("user {} created", *user.username);
            Ok(id)
        }
        Err(RepositoryError::Conflict) => {
            tracing::warn!("user {} already exists", *user.username);
            Err(ApiError::Conflict {
                trace_id: trace_id.clone(),
            })
        }
        Err(err) => {
            tracing::error!("failed to create user: {}", err);
            Err(ApiError::Unknown {
                trace_id: trace_id.clone(),
            })
        }
    }
}