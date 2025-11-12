use std::{collections::HashMap, sync::Arc};
use axum::{
    Json,
    Extension,
    extract::{Path, State, Query}
};
use crate::{
    AppState,
    error::ApiError,
    repositories::chats::ChatsRepository,
    services::{auth::Auth, trace::TraceId},
    models::{
        chats::ChatId,
        users::UserId,
        events::{
            SseEvent,
            SseEventType,
            MessageEvent,
        },
        messages::{
            MessageId,
            GetMessagesParams,
            NewMessageRequest,
            NewMessageResponse,
            GetMessagesResponse,
        },
    },
};

const MAX_MESSAGES: i64 = 100;

/// Send message to chat
#[utoipa::path(
    post,
    path = "/chats/{chat_id}",
    tag = "messages",
    params(
        ("chat_id" = ChatId, Path, description = "Chat id")
    ),
    request_body = NewMessageRequest,
    responses(
        (status = OK, description = "Message sent", body = NewMessageResponse, example = json!(NewMessageResponse { message_id: MessageId::from(84735) })),
        (status = BAD_REQUEST, description = "Most likely, you have specified empty content", example = json!({"type": "ValidationError", "trace_id": TraceId::new()})),
        (status = FORBIDDEN, description = "Forbidden", example = json!({"type": "Forbidden", "trace_id": TraceId::new()})),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error", example = json!({"type": "Unknown", "trace_id": TraceId::new()}))
    ),
    security(("auth" = []))
)]
pub async fn new_message(
    Extension(auth): Extension<Arc<Auth>>,
    Extension(trace_id): Extension<TraceId>,
    State(state): State<Arc<AppState>>,
    Path(chat_id): Path<ChatId>,
    Json(req): Json<NewMessageRequest>,
) -> Result<NewMessageResponse, ApiError> {
    tracing::trace!("new message for chat {chat_id} from user {}", auth.user.id);

    if !check_chat_access(&*state.chats, auth.user.id, chat_id).await {
        tracing::error!("forbidden");
        return Err(ApiError::Forbidden { trace_id });
    }

    let mut errors = HashMap::new();
    let content_errors = req.content.validate();

    if !content_errors.is_empty() {
        errors.insert("content".to_string(), content_errors);
    }

    if !errors.is_empty() {
        return Err(ApiError::Validation {
            trace_id,
            fields: errors,
        });
    }

    let message = match state
        .messages
        .create_message(chat_id, auth.user.id, req.content.as_ref())
        .await
    {
        Ok(message) => {
            tracing::trace!("message created with id {}", *message.id);
            message
        }
        Err(e) => {
            tracing::error!("failed to create message: {e}");
            return Err(ApiError::Unknown { trace_id });
        }
    };

    let chat_members = state.chats.get_chat_members(chat_id).await.map_err(|e| {
        tracing::error!("failed to get chat members: {e}");
        ApiError::Unknown { trace_id }
    })?;

    for member in chat_members {
        if member == auth.user.id {
            continue;
        }

        if let Some(member) = state.events.get(&member) {
            if let Err(e) = member.send(SseEvent::new(
                SseEventType::Message,
                MessageEvent {
                    user_id: auth.user.id,
                    message: message.clone(),
                    chat_id,
                },
            )) {
                tracing::error!("failed to send message event: {e}");
            }
        }
    }

    Ok(NewMessageResponse {
        message_id: message.id,
    })
}

#[tracing::instrument(skip(chats), ret)]
async fn check_chat_access(chats: &dyn ChatsRepository, user_id: UserId, chat_id: ChatId) -> bool {
    let Ok(chats) = chats.get_user_chats_ids(user_id).await else {
        return false;
    };

    chats.contains(&chat_id)
}

// /// Get chat messages
#[utoipa::path(
    get,
    path = "/chats/{chat_id}",
    tag = "messages",
    params(
        ("limit" = i64, Query, description = "Number of messages to return, max 100"),
        ("last_message_id" = Option<MessageId>, Query, description = "Last message id to return"),
        ("chat_id" = ChatId, Path, description = "Chat id")
    ),
    responses(
        (status = OK, description = "Messages ", body = GetMessagesResponse),
        (status = FORBIDDEN, description = "Forbidden", example = json!({"type": "Forbidden", "trace_id": TraceId::new()}))
    ),
    security(("auth" = []))
)]
pub async fn get_messages(
    Extension(auth): Extension<Arc<Auth>>,
    Extension(trace_id): Extension<TraceId>,
    State(state): State<Arc<AppState>>,
    Path(chat_id): Path<ChatId>,
    Query(params): Query<GetMessagesParams>,
) -> Result<GetMessagesResponse, ApiError> {
    if !check_chat_access(&*state.chats, auth.user.id, chat_id).await {
        tracing::error!("forbidden");
        return Err(ApiError::Forbidden { trace_id });
    }

    let limit = if params.limit > MAX_MESSAGES {
        MAX_MESSAGES
    } else {
        params.limit
    };

    let mut messages = state
        .messages
        .get_messages(chat_id, limit + 1, params.last_message_id)
        .await
        .map_err(|e| {
            tracing::error!("failed to get messages: {e}");
            ApiError::Unknown { trace_id }
        })?;

    let has_more = messages.len() > limit as usize;

    messages.truncate(limit as usize);

    Ok(GetMessagesResponse { messages, has_more })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use std::collections::HashSet;
    use crate::repositories::chats::MockChatsRepository;

    #[test]
    async fn test_check_chat_access_ok() {
        let mut chats = MockChatsRepository::new();
        chats.expect_get_user_chats_ids().returning(|_| {
            let set = HashSet::from([ChatId::new(1), ChatId::new(2), ChatId::new(3)]);
            Ok(set)
        });
        let user_id = UserId::new(1);
        let chat_id = ChatId::new(2);

        assert!(check_chat_access(&chats, user_id, chat_id).await);
    }

    #[test]
    async fn test_check_chat_access_fail() {
        let mut chats = MockChatsRepository::new();
        chats.expect_get_user_chats_ids().returning(|_| {
            let set = HashSet::from([ChatId::new(1), ChatId::new(2), ChatId::new(3)]);
            Ok(set)
        });
        let user_id = UserId::new(1);
        let chat_id = ChatId::new(4);

        assert!(!check_chat_access(&chats, user_id, chat_id).await);
    }
}