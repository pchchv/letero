use std::ops::Deref;
use serde::{Deserialize, Serialize};

pub const SESSION_LIFETIME: i64 = 60 * 60 * 24 * 7;

#[derive(Deserialize)]
pub struct Username(String);

impl Username {
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

#[derive(Deserialize)]
pub struct Password(String);

impl Password {
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

#[derive(Clone, Copy, Serialize, Deserialize, Debug, sqlx::Type)]
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
    pub created_at: time::UtcDateTime,
}