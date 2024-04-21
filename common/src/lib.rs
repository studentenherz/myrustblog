use bson::{oid::ObjectId, serde_helpers::chrono_datetime_as_bson_datetime};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod utils;

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
    pub max_age: u64,
}

#[derive(Deserialize, Serialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct PostCreatedResponse {
    pub slug: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdatePostRequest {
    pub slug: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub author: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub published_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct PostsQueryParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GetPostsResponse {
    pub posts: Vec<Post>,
    pub pages: Result<u64, ()>,
}
