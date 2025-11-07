use sqlx::{PgPool, query, query_as, query_scalar};
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

    async fn get_user_chats_ids(
        &self,
        user_id: UserId,
    ) -> Result<HashSet<ChatId>, RepositoryError> {
        let chat_ids = query_scalar!(
            "SELECT ChatId FROM ChatMembers WHERE UserId = $1",
            user_id as _
        )
        .fetch(&self.0)
        .filter_map(|id| async { id.map(ChatId::from).ok() })
        .collect::<HashSet<ChatId>>()
        .await;

        Ok(chat_ids)
    }

    async fn get_user_chats(&self, user_id: UserId) -> Result<Vec<Chat>, RepositoryError> {
        let chats = query_as!(
            Chat,
            "SELECT c.Id, c.Title as \"title: _\", array_agg(cm.UserId) AS \"users_ids!: _\"
            FROM Chats c JOIN ChatMembers cm ON c.Id = cm.ChatId
            WHERE c.Id IN (SELECT ChatId FROM ChatMembers WHERE UserId = $1)
            GROUP BY c.Id",
            user_id as _
        )
        .fetch_all(&self.0)
        .await?;

        Ok(chats)
    }
}
