use sqlx::PgPool;
use std::collections::HashSet;
use crate::{
    error::RepositoryError,
    models::{
        chats::{Chat, ChatId, ChatTitle},
        users::UserId,
    },
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait ChatsRepository: Send + Sync {
    async fn create_chat(
        &self,
        title: &ChatTitle,
        users: &[UserId],
    ) -> Result<ChatId, RepositoryError>;

    async fn remove_chat(&self, chat_id: ChatId) -> Result<(), RepositoryError>;
    async fn get_user_chats(&self, user_id: UserId) -> Result<Vec<Chat>, RepositoryError>;
    async fn get_user_chats_ids(&self, user_id: UserId)
    -> Result<HashSet<ChatId>, RepositoryError>;
    async fn get_chat_members(&self, chat_id: ChatId) -> Result<Vec<UserId>, RepositoryError>;
}

pub struct PgChatsRepository(PgPool);

impl PgChatsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}