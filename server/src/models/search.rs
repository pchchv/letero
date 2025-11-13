use crate::models::users::PublicUser;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct SearchUsersResponse(pub Vec<PublicUser>);

impl IntoResponse for SearchUsersResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}