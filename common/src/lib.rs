use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub max_age: u64,
}
