use crate::{error::RepositoryError, models::users::{PasswordHash, UserId}};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait UsersRepository: Send + Sync {
    async fn create_user(&self, username: &str, password: PasswordHash) -> Result<UserId, RepositoryError>;
    async fn get_user(&self, username: &str, password: PasswordHash) -> Result<User, RepositoryError>;
    async fn get_user_by_id(&self, id: &UserId) -> Result<User, RepositoryError>;
}

pub struct PgUsersRepository(sqlx::PgPool);

impl PgUsersRepository {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self(db)
    }
}

#[async_trait::async_trait]
impl UsersRepository for PgUsersRepository {
    async fn create_user(&self, username: &str, password: PasswordHash) -> Result<UserId, RepositoryError> {
        let result = sqlx::query_scalar!(
            "INSERT INTO Users (Name, Password, Salt) VALUES ($1, $2, $3) RETURNING Id",
            username,
            *password,
            password.get_salt(),
        )
        .fetch_one(&self.0)
        .await?;

        Ok(UserId::new(result))
    }

    async fn get_user(&self, username: &str, password: PasswordHash) -> Result<User, RepositoryError> {
        let result = sqlx::query_as!(
            User,
            "SELECT Id, Name as username, Password, CreatedAt as created_at FROM Users WHERE Name = $1 AND Password = $2",
            username,
            *password
        )
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }

    async fn get_user_by_id(&self, id: &UserId) -> Result<User, RepositoryError> {
        let result = sqlx::query_as!(User, "SELECT Id, Name as username, Password, CreatedAt as created_at FROM Users WHERE Id = $1", **id)
            .fetch_one(&self.0)
            .await?;

        Ok(result)
    }
}