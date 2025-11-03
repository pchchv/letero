use crate::services::trace::TraceId;
use serde::Serialize;
use std::{collections::HashMap, fmt::Display};
use axum::{Json, http::StatusCode, response::IntoResponse};

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum ApiError {
    Unknown {
        trace_id: TraceId,
    },
    Internal,
    Validation {
        fields: HashMap<String, Vec<String>>,
        trace_id: TraceId,
    },
    Conflict {
        trace_id: TraceId,
    },
    NotFound {
        trace_id: TraceId,
    },
    Unauthorized {
        trace_id: TraceId,
    },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            ApiError::Unknown { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Validation { .. } => StatusCode::BAD_REQUEST,
            ApiError::Conflict { .. } => StatusCode::CONFLICT,
            ApiError::NotFound { .. } => StatusCode::NOT_FOUND,
            ApiError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(self)).into_response()
    }
}

pub enum RepositoryError {
    Unknown(sqlx::Error),
    Conflict,
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::Unknown(err) => write!(f, "unknown repository error: {err}"),
            RepositoryError::Conflict => write!(f, "conflict"),
        }
    }
}

impl From<sqlx::Error> for RepositoryError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Database(ref db)
                if db.kind() == sqlx::error::ErrorKind::UniqueViolation =>
            {
                RepositoryError::Conflict
            }
            _ => RepositoryError::Unknown(err),
        }
    }
}