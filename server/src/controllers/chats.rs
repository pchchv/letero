use std::{
    sync::Arc,
    collections::HashMap,
};
use axum::{
    Json,
    Extension,
    extract::{State, Path},
};
use crate::{
    AppState,
    error::ApiError,
    services::{
        auth::Auth,
        trace::TraceId,
    },
    models::{
        users::UserId,
        events::{
            SseEvent,
            ChatEvent,
            SseEventType,
        },
        chats::{
            ChatId,
            ChatTitle,
            NewChatRequest,
            NewChatResponse,
            GetChatsResponse,
            RemoveChatResponse,
        },
    },
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

/// Create new chat
#[utoipa::path(
    post,
    path = "/chats",
    tag = "chats",
    request_body = NewChatRequest,
    responses(
        (status = OK, description = "Chat created", body = NewChatResponse),
        (status = BAD_REQUEST, description = "Validation error", example = json!({"type": "Validation", "fields": {"title": "Chat title is required"}, "trace_id": "aa23dcd356c"})),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error", example = json!({"type": "Internal", "trace_id": "aa23dcd356c"}))
    ),
    security(("auth" = []))
)]
pub async fn new_chat(
    Extension(auth): Extension<Arc<Auth>>,
    Extension(trace_id): Extension<TraceId>,
    State(state): State<Arc<AppState>>,
    Json(chat): Json<NewChatRequest>,
) -> Result<NewChatResponse, ApiError> {
    let errors = validate_chat(&chat.title);
    if !errors.is_empty() {
        return Err(ApiError::Validation {
            fields: errors,
            trace_id,
        });
    }

    let users_ids = if let Some(users_ids) = chat.users_ids
        && !users_ids.is_empty()
    {
        merge_ids(auth.user.id, users_ids)
    } else {
        vec![auth.user.id]
    };

    let chat_id = match state.chats.create_chat(&chat.title, &users_ids).await {
        Ok(id) => {
            tracing::trace!("chat {id} created");
            id
        }

        Err(err) => {
            tracing::error!("failed to create chat: {err}");
            return Err(ApiError::Unknown { trace_id });
        }
    };

    for member in &users_ids {
        if *member == auth.user.id {
            continue;
        }

        if let Some(member) = state.events.get(member)
            && let Err(err) = member.send(SseEvent::new(
                SseEventType::Chat,
                ChatEvent {
                    title: ChatTitle::new(chat.title.clone()),
                    users_ids: users_ids.clone(),
                    chat_id,
                },
            )) {
                tracing::error!("failed to send event: {err}");
            }
    }

    Ok(NewChatResponse::new(chat_id))
}

/// Remove chat
#[utoipa::path(
    delete,
    path = "/chats/{chat_id}",
    tag = "chats",
    params(
        ("chat_id" = ChatId, Path, description = "Chat id")
    ),
    responses(
        (status = NO_CONTENT, description = "Chat removed", body = RemoveChatResponse),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError, example = json!({"type": "Internal", "trace_id": "aa23dcd356c"}))
    ),
    security(("auth" = []))
)]
pub async fn remove_chat(
    Extension(_): Extension<Arc<Auth>>,
    Extension(trace_id): Extension<TraceId>,
    State(state): State<Arc<AppState>>,
    Path(chat_id): Path<ChatId>,
) -> Result<RemoveChatResponse, ApiError> {
    match state.chats.remove_chat(chat_id).await {
        Ok(_) => {
            tracing::trace!("chat {} removed", chat_id);
            Ok(RemoveChatResponse)
        }
        Err(err) => {
            tracing::error!("failed to remove chat: {err}");
            Err(ApiError::Unknown { trace_id })
        }
    }
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