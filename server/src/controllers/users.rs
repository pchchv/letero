use crate::models::users::NewUserRequest;
use std::collections::HashMap;

fn validate_user(user: &NewUserRequest) -> HashMap<String, Vec<String>> {
    let mut errors = HashMap::new();
    let username_errors = user.username.validate();
    if !username_errors.is_empty() {
        tracing::trace!("invalid username: {}", username_errors.join(", "));
        errors.insert("username".to_owned(), username_errors);
    }

    let password_errors = user.password.validate();
    if !password_errors.is_empty() {
        tracing::trace!("invalid password: {}", password_errors.join(", "));
        errors.insert("password".to_owned(), password_errors);
    }

    errors
}