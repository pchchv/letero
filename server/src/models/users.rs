use sha2::Digest;
use std::ops::Deref;
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};
use axum::{Json, http::StatusCode, response::IntoResponse};
use crate::services::auth::SESSION_COOKIE_NAME;

pub const SESSION_LIFETIME: i64 = 60 * 60 * 24 * 7;

#[derive(Deserialize, ToSchema)]
pub struct Username(String);

impl Username {
    pub fn new<I: Into<String>>(username: I) -> Self {
        Self(username.into())
    }

    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        let trim = self.0.trim();

        if trim.is_empty() {
            errors.push("Empty username".to_owned());
            return errors;
        }

        if trim.len() > 30 {
            errors.push("Username must be less than 30 characters".to_owned());
        }

        if trim.len() < 3 {
            errors.push("Username must be more than 3 characters".to_owned());
        }

        for ch in trim.chars() {
            if !(ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '.') {
                errors.push("Username must contain only latin letters or digits, underscores, dashes and dots".to_owned());
                return errors;
            }
        }

        errors
    }
}

impl Deref for Username {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize, ToSchema)]
pub struct Password(String);

impl Password {
    pub fn new<I: Into<String>>(password: I) -> Self {
        Self(password.into())
    }

    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        let trim = self.0.trim();

        if trim.is_empty() {
            errors.push("Empty password".to_owned());
            return errors;
        }

        if trim.len() < 6 {
            errors.push("Password must be more than 6 characters".to_owned());
        }

        errors
    }
}

impl Deref for Password {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, sqlx::Type, PartialEq, Eq, Hash, ToSchema)]
#[sqlx(transparent)]
pub struct UserId(i32);

impl UserId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }
}

impl From<i32> for UserId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl PartialEq<i32> for UserId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for UserId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub password: String,
    pub created_at: time::OffsetDateTime,
}

#[derive(Deserialize, ToSchema)]
pub struct LoginUserRequest {
    pub username: Username,
    pub password: Password,
}

#[derive(Serialize, ToSchema)]
pub struct LoginUserResponse {
    pub user_id: UserId,
    #[serde(skip)]
    pub session: String,
}

impl LoginUserResponse {
    pub fn new(user_id: UserId, session: String) -> Self {
        Self { user_id, session }
    }
}

impl IntoResponse for LoginUserResponse {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::CREATED,
            [(
                "Set-Cookie",
                format!(
                    "{SESSION_COOKIE_NAME}={}, Max-Age={SESSION_LIFETIME}",
                    self.session
                ),
            )],
            Json(self),
        )
            .into_response()
    }
}

pub struct PasswordHash(String, String);

impl PasswordHash {
    pub fn new(password: &str, salt: &str) -> Self {
        let hash = sha2::Sha256::digest(format!("{password}{salt}").as_bytes());
        Self(hex::encode(hash), salt.to_owned())
        }

    pub fn get_salt(&self) -> &str {
        &self.1
    }
}

impl Deref for PasswordHash {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(ToSchema)]
pub struct LogoutUserResponse;

impl IntoResponse for LogoutUserResponse {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::OK,
            [("Set-Cookie", format!("{SESSION_COOKIE_NAME}=_; Max-Age=0"))],
            (),
        )
            .into_response()
    }
}


#[derive(Serialize, ToSchema)]
pub struct PublicUser {
    pub id: UserId,
    pub username: String,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: time::OffsetDateTime,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
        }
    }
}