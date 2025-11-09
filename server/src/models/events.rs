use strum::AsRefStr;
use serde::Serialize;
use serde_json::to_string;
use crate::models::{users::UserId, chats::{ChatId, ChatTitle}};

#[derive(Clone, AsRefStr)]
pub enum SseEventType {
    Message,
    Chat,
}

#[derive(Clone)]
pub struct SseEvent {
    pub event_type: SseEventType,
    pub data: String,
}

impl SseEvent {
    pub fn new(event_type: SseEventType, data: impl Serialize) -> Self {
        let data = to_string(&data).expect("failed to serialize event data");
        Self { event_type, data }
    }
}

#[derive(Serialize)]
pub struct ChatEvent {
    pub chat_id: ChatId,
    pub title: ChatTitle,
    pub users_ids: Vec<UserId>,
}