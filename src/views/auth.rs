use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{_entities::users, user_auths};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &users::Model, user_auth: &user_auths::Model, token: &String) -> Self {
        Self {
            token: token.to_string(),
            pid: user_auth.id.to_string(), //pid: user.pid.to_string(),
            name: user.name.clone(),
            is_verified: user_auth.email_verified_at.is_some(), //is_verified: user.email_verified_at.is_some(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub pid: String,
    pub name: String,
    pub email: String,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &users::Model, pid: Uuid) -> Self {
        Self {
            pid: pid.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }
}
