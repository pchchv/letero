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
    async fn get_session(&self, uid: &str) -> Result<i32, RepositoryError>;
    async fn get_session_by_user_id(&self, user_id: UserId) -> Result<String, RepositoryError>;
    async fn remove_session(&self, uid: &str) -> Result<(), RepositoryError>;
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

    async fn get_session(&self, uid: &str) -> Result<i32, RepositoryError> {
        let row = sqlx::query!("SELECT UserId FROM Sessions WHERE Uid = $1", uid)
            .fetch_one(&self.0)
            .await?;

        Ok(row.userid)
    }

        async fn get_session_by_user_id(&self, user_id: UserId) -> Result<String, RepositoryError> {
        let uid = sqlx::query_scalar!("SELECT Uid FROM Sessions WHERE UserId = $1", user_id as _)
            .fetch_one(&self.0)
            .await?;

        Ok(uid)
    }

    async fn remove_session(&self, uid: &str) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM Sessions WHERE Uid = $1", uid)
            .execute(&self.0)
            .await?;

        Ok(())
    }
}