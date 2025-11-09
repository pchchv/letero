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