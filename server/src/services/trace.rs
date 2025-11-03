use serde::Serialize;
use std::{fmt::Display, ops::Deref, sync::Arc};

#[derive(Serialize, Clone, Debug, utoipa::ToSchema)]
pub struct TraceId(Arc<String>);

impl TraceId {
    pub fn new() -> TraceId {
        TraceId(Arc::new(small_uid::SmallUid::new().to_string()))
    }
}

impl Display for TraceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for TraceId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}