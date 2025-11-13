use crate::{error::RepositoryError, models::users::UserId};
use time::OffsetDateTime;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait SessionsRepository: Send + Sync {
    async fn create_session(
        &self,
        uid: &str,
        user_id: UserId,
        expires_at: OffsetDateTime,
    ) -> Result<(), RepositoryError>;
}

pub struct PgSessionsRepository(sqlx::PgPool);

impl PgSessionsRepository {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self(db)
    }
}

#[async_trait::async_trait]
impl SessionsRepository for PgSessionsRepository {
    async fn create_session(
        &self,
        uid: &str,
        user_id: UserId,
        expires_at: OffsetDateTime,
    ) -> Result<(), RepositoryError> {
        sqlx::query!(
            "INSERT INTO Sessions (Uid, UserId, ExpiresAt) VALUES ($1, $2, $3)",
            uid,
            user_id as _,
            expires_at
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }
}