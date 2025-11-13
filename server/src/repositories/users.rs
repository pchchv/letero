use tokio_stream::StreamExt;
use crate::{error::RepositoryError, models::users::{PasswordHash, User, UserId}};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait UsersRepository: Send + Sync {
    async fn create_user(&self, username: &str, password: PasswordHash) -> Result<UserId, RepositoryError>;
    async fn get_user(&self, username: &str, password: PasswordHash) -> Result<User, RepositoryError>;
    async fn get_user_by_id(&self, id: &UserId) -> Result<User, RepositoryError>;
    async fn get_user_by_session(&self, session_id: &str) -> Result<User, RepositoryError>;
    async fn search_users_by_username(&self, username: &str) -> Result<Vec<User>, RepositoryError>;
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

    async fn get_user_by_session(&self, session_id: &str) -> Result<User, RepositoryError> {
        let result = sqlx::query_as!(User, 
            "SELECT u.Id, u.Name as username, u.Password, u.CreatedAt as created_at FROM Users u LEFT JOIN Sessions s ON u.Id = s.UserId WHERE s.Uid = $1", 
            session_id
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

    async fn search_users_by_username(&self, username: &str) -> Result<Vec<User>, RepositoryError> {
        let result = sqlx::query!( 
                "SELECT *, similarity(name, $1) AS sim 
                FROM Users 
                WHERE Name % $1
                ORDER BY sim DESC
                LIMIT 5", username)
            .fetch(&self.0)
            .filter_map(|row| {
                match row {
                    Ok(row) => Some(User {
                        id: UserId::new(row.id),
                        username: row.name,
                        password: row.password,
                        created_at: row.createdat,
                    }),
                    Err(err) => {
                        tracing::error!("failed to search users: {}", err);
                        None
                    }
                }
            })
            .collect::<Vec<_>>()
            .await;

        Ok(result)
    }
}