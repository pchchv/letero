use crate::repositories::{sessions::SessionsRepository, users::UsersRepository};
use std::sync::Arc;

pub struct AppState {
    pub users: Arc<dyn UsersRepository>,
    pub sessions: Arc<dyn SessionsRepository>,
}