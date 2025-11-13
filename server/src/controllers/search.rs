use crate::{
    AppState,
    error::ApiError,
    services::trace::TraceId,
    models::search::{SearchUsersQuery, SearchUsersResponse},
};
use axum::{
    Extension,
    extract::{Query, State},
};
use std::{collections::HashMap, sync::Arc};

/// Search users
#[utoipa::path(
    get,
    path = "/search/users",
    tags = ["users", "search"],
    params(
        ("username" = String, Query, description = "Username to search for")
    ),
    responses(
        (status = OK, description = "Users found", body = SearchUsersResponse),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError)
    )
)]
pub async fn search_users(
    Extension(trace_id): Extension<TraceId>,
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchUsersQuery>,
) -> Result<SearchUsersResponse, ApiError> {
    let errors = params.username.validate();
    if !errors.is_empty() {
        return Err(ApiError::Validation {
            fields: HashMap::from([("username".to_string(), errors)]),
            trace_id,
        });
    }

    let users = state.users.search_users_by_username(&params.username).await;
    match users {
        Ok(users) => Ok(SearchUsersResponse(
            users.into_iter().map(|user| user.into()).collect(),
        )),

        Err(err) => {
            tracing::error!("failed to search users: {}", err);
            Err(ApiError::Unknown {
                trace_id: trace_id.clone(),
            })
        }
    }
}