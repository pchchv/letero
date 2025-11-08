use crate::models::users::User;

pub const SESSION_COOKIE_NAME: &str = "session";

pub struct Auth {
    pub session: String,
    pub user: User,
}