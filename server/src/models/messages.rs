use crate::models::{chats::ChatId, users::UserId};
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::ops::Deref;

#[derive(Serialize, Debug, ToSchema, Clone)]
pub struct Message {
    pub id: MessageId,
    pub content: String,
    pub chat_id: ChatId,
    pub sender_id: Option<UserId>,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: time::OffsetDateTime,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, sqlx::Type, PartialEq, ToSchema)]
#[sqlx(transparent)]
pub struct MessageId(i64);

impl From<i64> for MessageId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl Deref for MessageId {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, ToSchema)]
pub struct GetMessagesResponse {
    pub messages: Vec<Message>,
    pub has_more: bool,
}

impl IntoResponse for GetMessagesResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}