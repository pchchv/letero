use::std::sync::Arc;
use std::collections::HashMap;
use axum::{Extension, extract::State};
use crate::{
    AppState,
    error::ApiError,
    models::{users::UserId, chats::{ChatTitle, GetChatsResponse}},
    services::{auth::Auth, trace::TraceId},
};

/// Get user chats
#[utoipa::path(
    get,
    path = "/chats",
    tag = "chats",
    responses(
        (status = OK, description = "User chats", body = GetChatsResponse),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError, example = json!({"type": "Internal", "trace_id": "aa23dcd356c"}))
    ),
    security(("auth" = []))
)]
pub async fn get_chats(
    Extension(auth): Extension<Arc<Auth>>,
    Extension(trace_id): Extension<TraceId>,
    State(state): State<Arc<AppState>>,
) -> Result<GetChatsResponse, ApiError> {
    tracing::trace!("getting chats for user {}", auth.user.id);
    let chats = match state.chats.get_user_chats(auth.user.id).await {
        Ok(chats) => {
            tracing::trace!("user {} has {} chats", auth.user.id, chats.len());
            chats
        }

        Err(err) => {
            tracing::error!("failed to get user chats: {err}");
            return Err(ApiError::Unknown { trace_id });
        }
    };

    Ok(GetChatsResponse(chats))
}

fn validate_chat(title: &ChatTitle) -> HashMap<String, Vec<String>> {
    let mut errors = HashMap::new();
    let title_errors = title.validate();
    if !title_errors.is_empty() {
        errors.insert("title".to_owned(), title_errors);
    }

    errors
}

fn merge_ids(user_id: UserId, ids: Vec<UserId>) -> Vec<UserId> {
    let mut users = Vec::with_capacity(ids.len() + 1);
    users.push(user_id);
    users.extend(ids);
    users
}