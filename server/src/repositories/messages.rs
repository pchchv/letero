use sqlx::{PgPool, query_as};
use crate::{
    error::RepositoryError,
    models::{
        chats::ChatId,
        messages::{Message, MessageId},
        users::UserId,
    },
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait MessagesRepository: Send + Sync {
    async fn get_messages(
        &self,
        chat_id: ChatId,
        limit: i64,
        last_message_id: Option<MessageId>,
    ) -> Result<Vec<Message>, RepositoryError>;

    async fn create_message(
        &self,
        chat_id: ChatId,
        user_id: UserId,
        content: &str,
    ) -> Result<Message, RepositoryError>;
}

pub struct PgMessagesRepository(PgPool);

impl PgMessagesRepository {
    pub fn new(pool: PgPool) -> Self {
        PgMessagesRepository(pool)
    }
}

#[async_trait::async_trait]
impl MessagesRepository for PgMessagesRepository {
    async fn get_messages(
        &self,
        chat_id: ChatId,
        limit: i64,
        last_message_id: Option<MessageId>,
    ) -> Result<Vec<Message>, RepositoryError> {
        let result = query_as!(
                Message,
                "SELECT Id, UserId as \"sender_id: _\", ChatId as chat_id, Content, CreatedAt as created_at
                FROM messages
                WHERE chatid = $1 AND ($2::BIGINT IS NULL OR Id < $2)
                ORDER BY CreatedAt DESC
                LIMIT $3",
                chat_id as _,
                last_message_id as _,
                limit,
            ).fetch_all(&self.0)
            .await?;

        Ok(result)
    }

    async fn create_message(
        &self,
        chat_id: ChatId,
        user_id: UserId,
        content: &str,
    ) -> Result<Message, RepositoryError> {
        let message = query_as!(Message,
            "INSERT INTO Messages (ChatId, UserId, Content) 
            VALUES ($1, $2, $3) 
            RETURNING Id, UserId as \"sender_id: _\", ChatId as chat_id, Content, CreatedAt as created_at",
            chat_id as _,
            user_id as _,
            content
        )
        .fetch_one(&self.0)
        .await?;

        Ok(message)
    }
}