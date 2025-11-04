use crate::{
    rand::RandomGenerator,
    repositories::{
        sessions::{SessionsRepository, PgSessionsRepository},
        users::{UsersRepository, PgUsersRepository},
    },
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub random: Arc<Mutex<dyn RandomGenerator>>,
    pub users: Arc<dyn UsersRepository>,
    pub sessions: Arc<dyn SessionsRepository>,
}

impl AppState {
    pub fn new(random: Arc<Mutex<dyn RandomGenerator>>, pool: sqlx::PgPool) -> Self {
        Self {
            users: Arc::new(PgUsersRepository::new(pool.clone())),
            sessions: Arc::new(PgSessionsRepository::new(pool)),
            random,
        }
    }
}