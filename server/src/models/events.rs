use strum::AsRefStr;

#[derive(Clone, AsRefStr)]
pub enum SseEventType {
    Message,
    Chat,
}