use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, sqlx::Type, PartialEq, ToSchema)]
#[sqlx(transparent)]
pub struct MessageId(i64);

impl From<i64> for MessageId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl Deref for MessageId {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}