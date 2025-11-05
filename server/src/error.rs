use crate::services::{trace::TraceId, auth::SESSION_COOKIE_NAME};
use axum::{Json, http::StatusCode, response::IntoResponse};
use std::{collections::HashMap, fmt::Display};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
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
            ApiError::Unauthorized { .. } => {
                return (
                    StatusCode::UNAUTHORIZED,
                    [
                        ("Set-Cookie", format!("{SESSION_COOKIE_NAME}=_; Max-Age=0")),
                        ("Location", "/unauthorized".to_owned()),
                    ],
                    Json(self),
                ).into_response();
            }
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(self)).into_response()
    }
}

#[derive(Debug)]
pub enum RepositoryError {
    Unknown(sqlx::Error),
    Conflict,
    NotFound,
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::Unknown(err) => write!(f, "unknown repository error: {err}"),
            RepositoryError::Conflict => write!(f, "conflict"),
            RepositoryError::NotFound => write!(f, "not found"),
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
            sqlx::Error::RowNotFound => RepositoryError::NotFound,
            _ => RepositoryError::Unknown(err),
        }
    }
}