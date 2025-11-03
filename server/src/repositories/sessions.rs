#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait SessionsRepository: Send + Sync {
    async fn create_session(&self, uid: &str, user_id: i32) -> Result<(), sqlx::Error>;
}

pub struct PgSessionsRepository(sqlx::PgPool);

impl PgSessionsRepository {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self(db)
    }
}

#[async_trait::async_trait]
impl SessionsRepository for PgSessionsRepository {
    async fn create_session(&self, uid: &str, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO Sessions (Uid, UserId) VALUES ($1, $2)",
            uid,
            user_id
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }
}