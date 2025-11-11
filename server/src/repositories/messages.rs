use sqlx::PgPool;
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