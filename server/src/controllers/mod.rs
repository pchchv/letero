use std::ops::Deref;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginUserRequest {
    pub username: Username,
    pub password: Password,
}

#[derive(Deserialize)]
pub struct Username(pub String);

impl Username {
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        let trim = self.0.trim();

        if trim.is_empty() {
            errors.push("Empty username".to_owned());
            return errors;
        }

        if trim.len() > 30 {
            errors.push("Username must be less than 30 characters".to_owned());
        }

        if trim.len() < 3 {
            errors.push("Username must be more than 3 characters".to_owned());
        }

        for ch in trim.chars() {
            if !(ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '.') {
                errors.push("Username must contain only latin letters or digits, underscores, dashes and dots".to_owned());
                return errors;
            }
        }

        errors
    }
}

impl Deref for Username {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize)]
pub struct Password(pub String);

impl Password {
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        let trim = self.0.trim();

        if trim.is_empty() {
            errors.push("Empty password".to_owned());
            return errors;
        }

        if trim.len() < 6 {
            errors.push("Password must be more than 6 characters".to_owned());
        }

        errors
    }
}

impl Deref for Password {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}