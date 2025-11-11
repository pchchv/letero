use crate::{
    models::{events::SseEvent, users::UserId},
    rand::RandomGenerator,
    repositories::{
        chats::{ChatsRepository, PgChatsRepository},
        sessions::{SessionsRepository, PgSessionsRepository},
        users::{UsersRepository, PgUsersRepository},
    },
};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};

pub struct AppState {
    pub random: Arc<Mutex<dyn RandomGenerator>>,
    pub events: Arc<DashMap<UserId, broadcast::Sender<SseEvent>>>,
    pub users: Arc<dyn UsersRepository>,
    pub sessions: Arc<dyn SessionsRepository>,
    pub chats: Arc<dyn ChatsRepository>,
}

impl AppState {
    pub fn new(random: Arc<Mutex<dyn RandomGenerator>>, pool: sqlx::PgPool) -> Self {
        Self {
            users: Arc::new(PgUsersRepository::new(pool.clone())),
            sessions: Arc::new(PgSessionsRepository::new(pool.clone())),
            chats: Arc::new(PgChatsRepository::new(pool.clone())),
            events: Arc::new(DashMap::new()),
            random,
        }
    }
}