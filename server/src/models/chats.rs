use std::ops::Deref;
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, sqlx::Type, Serialize, ToSchema)]
#[sqlx(transparent)]
pub struct ChatTitle(String);

impl ChatTitle {
    pub fn new(title: String) -> Self {
        Self(title)
    }

    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.0.is_empty() {
            errors.push("Title is empty".to_string());
        }

        if self.0.len() > 50 {
            errors.push("Title is too long".to_string());
        }

        errors
    }
}

impl Deref for ChatTitle {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, ToSchema)]
#[sqlx(transparent)]
pub struct ChatId(i32);

impl ChatId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }
}

impl From<i32> for ChatId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for ChatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<i32> for ChatId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl Deref for ChatId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
