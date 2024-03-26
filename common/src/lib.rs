use serde::{Deserialize, Serialize};

pub mod utils;

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
    pub max_age: u64,
}
