use crate::{
    error::{ApiError, RepositoryError},
    models::users::{
        LoginUserRequest, LoginUserResponse, PasswordHash, UserId
    },
    rand::RandomGenerator,
    repositories::users::UsersRepository,
    services::trace::TraceId,
    state::AppState,
};
use axum::{extract::State, Extension, Json};
use std::{collections::HashMap, sync::Arc};

/// Create new user
#[utoipa::path(post,
    path = "/users",
    tag = "users", 
    request_body(
        content = LoginUserRequest, 
        description = "User credentials"),
    responses(
        (status = OK, description = "User created", body = LoginUserResponse),
        (status = BAD_REQUEST, description = "Invalid user credentials", body = ApiError),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError)
    )
)]
pub async fn new_user(
    Extension(trace_id): Extension<TraceId>,
    State(state): State<Arc<AppState>>,
    Json(user): Json<LoginUserRequest>,
) -> Result<LoginUserResponse, ApiError> {
    tracing::trace!("validationg user credentials");
    let errors = validate_user(&user);

    if !errors.is_empty() {
        tracing::trace!("invalid user credentials, returning error");
        return Err(ApiError::Validation {
            fields: errors,
            trace_id,
        });
    }

    let user_id = create_user(&state.random, &*state.users, &user, &trace_id).await?;
    let session = create_session(&*state.sessions, user_id, &trace_id).await?;

    Ok(LoginUserResponse::new(user_id, session))
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