use crate::error::RepositoryError;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait UsersRepository: Send + Sync {
    async fn create_user(&self, username: &str, password: &str) -> Result<i32, RepositoryError>;
}

pub struct PgUsersRepository(sqlx::PgPool);

impl PgUsersRepository {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self(db)
    }
}

#[async_trait::async_trait]
impl UsersRepository for PgUsersRepository {
    async fn create_user(&self, username: &str, password: &str) -> Result<i32, RepositoryError> {
        let result = sqlx::query!(
            "INSERT INTO Users (Name, Password) VALUES ($1, $2) RETURNING Id",
            username,
            password
        )
        .fetch_one(&self.0)
        .await?;

        Ok(result.id)
    }
}