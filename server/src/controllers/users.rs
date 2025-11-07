use axum::{extract::State, Extension, Json};
use std::{collections::HashMap, sync::Arc};
use time::{Duration, OffsetDateTime};
use crate::{
    error::{ApiError, RepositoryError},
    repositories::{sessions::SessionsRepository, users::UsersRepository},
    services::trace::TraceId,
    rand::RandomGenerator,
    state::AppState,
    models::users::{
        LoginUserRequest, LoginUserResponse, PasswordHash, UserId, Username, SESSION_LIFETIME
    },
};

/// Create new user
#[utoipa::path(post,
    path = "/users",
    tag = "users", 
    request_body(
        content = LoginUserRequest, 
        description = "User credentials"),
    responses(
        (status = OK, description = "User created", body = LoginUserResponse),
        (status = BAD_REQUEST, description = "Invalid user credentials", body = ApiError),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError)
    )
)]
pub async fn new_user(
    Extension(trace_id): Extension<TraceId>,
    State(state): State<Arc<AppState>>,
    Json(user): Json<LoginUserRequest>,
) -> Result<LoginUserResponse, ApiError> {
    tracing::trace!("validationg user credentials");
    let errors = validate_user(&user);

    if !errors.is_empty() {
        tracing::trace!("invalid user credentials, returning error");
        return Err(ApiError::Validation {
            fields: errors,
            trace_id,
        });
    }

    let user_id = create_user(&state.random, &*state.users, &user, &trace_id).await?;
    let session = create_session(&*state.sessions, user_id, &trace_id).await?;

    Ok(LoginUserResponse::new(user_id, session))
}

fn validate_user(user: &LoginUserRequest) -> HashMap<String, Vec<String>> {
    let mut errors = HashMap::new();
    let username_errors = user.username.validate();
    if !username_errors.is_empty() {
        tracing::trace!("invalid username: {}", username_errors.join(", "));
        errors.insert("username".to_owned(), username_errors);
    }

    let password_errors = user.password.validate();
    if !password_errors.is_empty() {
        tracing::trace!("invalid password: {}", password_errors.join(", "));
        errors.insert("password".to_owned(), password_errors);
    }

    errors
}

async fn create_user(
    rand: &tokio::sync::Mutex<dyn RandomGenerator>,
    users: &dyn UsersRepository,
    user: &LoginUserRequest,
    trace_id: &TraceId,
) -> Result<UserId, ApiError> {
    tracing::trace!("hashing password...");

    let salt = rand.lock().await.get_salt();
    let password_hash = PasswordHash::new(&user.password, &salt);

    tracing::trace!("saving user credintials in database...");
    let result = users.create_user(&user.username, password_hash).await;

    match result {
        Ok(id) => {
            tracing::info!("user {} created", *user.username);
            Ok(id)
        }
        Err(RepositoryError::Conflict) => {
            tracing::warn!("user {} already exists", *user.username);
            Err(ApiError::Conflict {
                trace_id: trace_id.clone(),
            })
        }
        Err(err) => {
            tracing::error!("failed to create user: {}", err);
            Err(ApiError::Unknown {
                trace_id: trace_id.clone(),
            })
        }
    }
}

async fn get_user_salt(
    users: &dyn UsersRepository,
    username: &Username,
    trace_id: &TraceId,
) -> Result<String, ApiError> {
    match users.get_user_salt(username).await {
        Ok(salt) => {
            tracing::info!("user {} salt found", **username);
            Ok(salt)
        }

        Err(RepositoryError::NotFound) => {
            tracing::warn!("user {} not found", **username);
            Err(ApiError::NotFound {
                trace_id: trace_id.clone(),
            })
        }

        Err(err) => {
            tracing::error!("failed to get user salt: {}", err);
            Err(ApiError::Unknown {
                trace_id: trace_id.clone(),
            })
        }
    }
}

async fn get_user_id(
    users: &dyn UsersRepository,
    user: &LoginUserRequest,
    trace_id: &TraceId,
) -> Result<UserId, ApiError> {
    tracing::trace!("getting user salt...");

    let salt = get_user_salt(users, &user.username, trace_id).await?;
    let password_hash = PasswordHash::new(&user.password, &salt);

    tracing::trace!("getting user id...");
    match users.get_user(&user.username, password_hash).await {
        Ok(user) => {
            tracing::info!("user {} found", user.username);
            Ok(user.id)
        }

        Err(RepositoryError::NotFound) => {
            tracing::warn!("user {} not found", *user.username);
            Err(ApiError::NotFound {
                trace_id: trace_id.clone(),
            })
        }

        Err(err) => {
            tracing::error!("failed to get user: {}", err);
            Err(ApiError::Unknown {
                trace_id: trace_id.clone(),
            })
        }
    }
}

async fn create_session(
    sessions: &dyn SessionsRepository,
    user_id: UserId,
    trace_id: &TraceId,
) -> Result<String, ApiError> {
    for _ in 0..5 {
        tracing::trace!("generating session UID...");
        let uid = small_uid::SmallUid::new().to_string();
        let expires_at = OffsetDateTime::now_utc().saturating_add(Duration::seconds(SESSION_LIFETIME));
        tracing::trace!("trying to save session UID {uid} for user id {user_id} in database...");
        match sessions.create_session(&uid, user_id, expires_at).await {
            Ok(_) => {
                tracing::info!("session {} created for user {}", uid, user_id);
                return Ok(uid);
            }
            Err(RepositoryError::Conflict) => {
                tracing::warn!("session UID {} collision for user {}", uid, user_id);
                continue;
            }
            Err(err) => {
                tracing::error!("failed to create {} session: {}", uid, err);
                return Err(ApiError::Unknown {
                    trace_id: trace_id.clone(),
                });
            }
        };
    }

    tracing::error!("failed to create session");
    Err(ApiError::Conflict {
        trace_id: trace_id.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Display;
    use tokio::{sync::Mutex, test};
    use crate::{
        error::RepositoryError,
        rand::MockRandomGenerator,
        models::users::{LoginUserRequest, Password, User, Username},
        repositories::{sessions::MockSessionsRepository, users::MockUsersRepository},
    };

        #[test]
    async fn test_create_user_ok() {
        let mut rand = MockRandomGenerator::new();
        rand.expect_get_salt().returning(|| "salt".to_string());
        let mut users = MockUsersRepository::new();
        users
            .expect_create_user()
            .returning(|_, _| Ok(UserId::new(1)));

        let user = LoginUserRequest {
            username: Username::new("valid_user"),
            password: Password::new("ValidPass123"),
        };

        let result = create_user(&Mutex::new(rand), &users, &user, &TraceId::new())
            .await
            .expect("failed to create user");
        assert_eq!(result, 1);
    }

    #[test]
    async fn test_create_user_conflict() {
        let rand = Mutex::new(MockRandomGenerator::new());
        rand.lock()
            .await
            .expect_get_salt()
            .returning(|| "salt".to_string());
        let mut users = MockUsersRepository::new();
        users
            .expect_create_user()
            .returning(|_, _| Err(RepositoryError::Conflict));

        let user = LoginUserRequest {
            username: Username::new("valid_user"),
            password: Password::new("ValidPass123"),
        };

        let result = create_user(&rand, &users, &user, &TraceId::new()).await;
        assert!(matches!(result, Err(ApiError::Conflict { .. })));
    }

    #[test]
    async fn test_validate_user_ok() {
        let user = LoginUserRequest {
            username: Username::new("valid_user"),
            password: Password::new("ValidPass123"),
        };

        let errors = validate_user(&user);
        assert!(errors.is_empty());
    }

    #[test]
    async fn test_validate_user_invalid_username_and_password() {
        let user = LoginUserRequest {
            username: Username::new(""),
            password: Password::new(""),
        };

        let errors = validate_user(&user);
        assert!(errors.contains_key("username"));
        assert!(errors.contains_key("password"));
    }

    #[test]
    async fn test_validate_user_invalid_username() {
        let user = LoginUserRequest {
            username: Username::new("вууу"),
            password: Password::new("ValidPass123"),
        };

        let errors = validate_user(&user);
        assert!(errors.contains_key("username"));
    }

    #[test]
    async fn test_validate_user_invalid_username_length() {
        let user = LoginUserRequest {
            username: Username::new("0123456789012345678901234567890123456789"),
            password: Password::new("123"),
        };

        let errors = validate_user(&user);
        assert!(errors.contains_key("username"));
    }

    #[test]
    async fn test_get_user_id_by_username_ok() {
        let mut users = MockUsersRepository::new();
        users
            .expect_get_user_salt()
            .returning(|_| Ok("salt".to_string()));
        users.expect_get_user().returning(|username, password| {
            Ok(User {
                id: UserId::new(1),
                username: username.into(),
                password: password.to_string(),
                created_at: OffsetDateTime::now_utc(),
            })
        });

        let result = get_user_id(
            &users,
            &LoginUserRequest {
                username: Username::new("valid_user"),
                password: Password::new("ValidPass123"),
            },
            &TraceId::new(),
        )
        .await;

        assert!(result.is_ok());
    }

    #[test]
    async fn test_get_user_id_by_username_not_found() {
        let mut users = MockUsersRepository::new();
        users
            .expect_get_user_salt()
            .returning(|_| Ok("salt".to_string()));
        users
            .expect_get_user()
            .returning(|_, _| Err(RepositoryError::NotFound));

        let result = get_user_id(
            &users,
            &LoginUserRequest {
                username: Username::new("valid_user"),
                password: Password::new("ValidPass123"),
            },
            &TraceId::new(),
        )
        .await;

        assert!(matches!(result, Err(ApiError::NotFound { .. })));
    }

    #[test]
    async fn test_create_session_ok() {
        let mut sessions = MockSessionsRepository::new();
        sessions.expect_create_session().returning(|_, _, _| Ok(()));

        let result = create_session(&sessions, UserId::new(1), &TraceId::new()).await;
        assert!(result.is_ok());
    }

    #[test]
    async fn test_create_session_conflict() {
        let mut sessions = MockSessionsRepository::new();
        sessions
            .expect_create_session()
            .returning(|_, _, _| Err(RepositoryError::Conflict));

        let result = create_session(&sessions, UserId::new(1), &TraceId::new()).await;
        assert!(matches!(result, Err(ApiError::Conflict { .. })));
    }

    #[test]
    async fn test_create_session_unknown() {
        #[derive(Debug)]
        struct UnknownSqlxError;

        impl Display for UnknownSqlxError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "unknown sqlx error")
            }
        }

        impl std::error::Error for UnknownSqlxError {}

        impl sqlx::error::DatabaseError for UnknownSqlxError {
            fn message(&self) -> &str {
                todo!()
            }

            fn as_error(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
                todo!()
            }

            fn as_error_mut(&mut self) -> &mut (dyn std::error::Error + Send + Sync + 'static) {
                todo!()
            }

            fn into_error(self: Box<Self>) -> Box<dyn std::error::Error + Send + Sync + 'static> {
                todo!()
            }

            fn kind(&self) -> sqlx::error::ErrorKind {
                sqlx::error::ErrorKind::Other
            }
        }

        let mut sessions = MockSessionsRepository::new();
        sessions.expect_create_session().returning(|_, _, _| {
            Err(RepositoryError::Unknown(sqlx::Error::Database(Box::new(
                UnknownSqlxError,
            ))))
        });

        let result = create_session(&sessions, UserId::new(1), &TraceId::new()).await;
        assert!(matches!(result, Err(ApiError::Unknown { .. })));
    }
}