use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::{Mutex, broadcast};
use crate::{
    models::{events::SseEvent, users::UserId},
    rand::RandomGenerator,
    repositories::{
        chats::{ChatsRepository, PgChatsRepository},
        users::{UsersRepository, PgUsersRepository},
        messages::{MessagesRepository, PgMessagesRepository},
        sessions::{SessionsRepository, PgSessionsRepository},
    },
};

pub struct AppState {
    pub random: Arc<Mutex<dyn RandomGenerator>>,
    pub events: Arc<DashMap<UserId, broadcast::Sender<SseEvent>>>,
    pub users: Arc<dyn UsersRepository>,
    pub sessions: Arc<dyn SessionsRepository>,
    pub chats: Arc<dyn ChatsRepository>,
    pub messages: Arc<dyn MessagesRepository>,
}

impl AppState {
    pub fn new(random: Arc<Mutex<dyn RandomGenerator>>, pool: sqlx::PgPool) -> Self {
        Self {
            users: Arc::new(PgUsersRepository::new(pool.clone())),
            sessions: Arc::new(PgSessionsRepository::new(pool.clone())),
            chats: Arc::new(PgChatsRepository::new(pool.clone())),
            messages: Arc::new(PgMessagesRepository::new(pool)),
            events: Arc::new(DashMap::new()),
            random,
        }
    }
}