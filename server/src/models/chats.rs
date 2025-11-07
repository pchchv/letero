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
