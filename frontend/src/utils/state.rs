use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub role: String,
}

#[derive(PartialEq, Debug, Default, Serialize, Deserialize, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct AppState {
    pub user: Option<User>,
}
