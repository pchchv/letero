use std::ops::Deref;
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};
use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use crate::models::users::UserId;

#[derive(Serialize, ToSchema)]
pub struct Chat {
    pub id: ChatId,
    pub title: ChatTitle,
    pub users_ids: Vec<UserId>,
}

#[derive(Deserialize, sqlx::Type, Serialize, ToSchema)]
#[sqlx(transparent)]
pub struct ChatTitle(String);

impl ChatTitle {
    pub fn new(title: String) -> Self {
        Self(title)
    }

    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.0.is_empty() {
            errors.push("Title is empty".to_string());
        }

        if self.0.len() > 50 {
            errors.push("Title is too long".to_string());
        }

        errors
    }
}

impl Deref for ChatTitle {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, ToSchema)]
#[sqlx(transparent)]
pub struct ChatId(i32);

impl ChatId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }
}

impl From<i32> for ChatId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for ChatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<i32> for ChatId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl Deref for ChatId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize, ToSchema)]
pub struct NewChatRequest {
    pub title: ChatTitle,
    pub users_ids: Option<Vec<UserId>>,
}

#[derive(Serialize, ToSchema)]
pub struct GetChatsResponse(pub Vec<Chat>);

impl IntoResponse for GetChatsResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize, ToSchema)]
pub struct NewChatResponse {
    pub chat_id: ChatId,
}

impl NewChatResponse {
    pub fn new(chat_id: ChatId) -> Self {
        Self { chat_id }
    }
}

impl IntoResponse for NewChatResponse {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}