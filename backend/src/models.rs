use std::convert::From;

use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub use common::{Post, PostsQueryParams};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegistration {
    pub username: String,
    pub email: String,
    pub password: String,
    pub host: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfirmation {
    pub confirmation_token: String,
}

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    pub password: String, // This will be hashed
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnconfirmedUser {
    pub confirmation_token: String,
    pub host: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    pub confirmed: bool,
    pub username: String,
    pub email: String,
    pub password: String, // This will be hashed
    pub role: String,
}

impl From<UnconfirmedUser> for User {
    fn from(value: UnconfirmedUser) -> Self {
        Self {
            id: None,
            username: value.username,
            email: value.email,
            password: value.password,
            role: value.role,
        }
    }
}
