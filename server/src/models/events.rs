use strum::AsRefStr;

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
    pub fn new(event_type: SseEventType, data: impl serde::Serialize) -> Self {
        let data = serde_json::to_string(&data).expect("failed to serialize event data");
        Self { event_type, data }
    }
}

#[derive(serde::Serialize)]
pub struct ChatEvent {
    pub chat_id: crate::models::chats::ChatId,
    pub title: crate::models::chats::ChatTitle,
    pub users_ids: Vec<crate::models::users::UserId>,
}