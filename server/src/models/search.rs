use crate::models::users::{PublicUser, Username};
use serde::{Deserialize, Serialize};
use axum::response::IntoResponse;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct SearchUsersResponse(pub Vec<PublicUser>);

impl IntoResponse for SearchUsersResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

#[derive(Deserialize, ToSchema)]
pub struct SearchUsersQuery {
    pub username: Username,
}
