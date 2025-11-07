use sqlx::{PgPool, query};
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

#[async_trait::async_trait]
impl ChatsRepository for PgChatsRepository {
    async fn create_chat(
        &self,
        title: &ChatTitle,
        users: &[UserId],
    ) -> Result<ChatId, RepositoryError> {
        let mut tn = self.0.begin().await?;

        let chat_id = ChatId::new(
            sqlx::query_scalar!(
                "INSERT INTO Chats (Title) VALUES ($1) RETURNING Id",
                title as _
            )
            .fetch_one(&mut *tn)
            .await?,
        );

        query!(
            "INSERT INTO ChatMembers (ChatId, UserId) SELECT $1, UNNEST($2::int[])",
            chat_id as _,
            users as _,
        )
        .execute(&mut *tn)
        .await?;

        tn.commit().await?;

        Ok(chat_id)
    }

    async fn remove_chat(&self, chat_id: ChatId) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM Chats WHERE Id = $1", chat_id as _)
            .execute(&self.0)
            .await?;
        Ok(())
    }
}
