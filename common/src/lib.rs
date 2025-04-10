use chrono::{DateTime, Utc};
use pulldown_cmark::HeadingLevel;
use serde::{Deserialize, Serialize};

pub mod utils;

#[derive(Deserialize, Serialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub public: bool,
}

#[derive(Deserialize, Serialize)]
pub struct PostCreatedResponse {
    pub slug: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdatePostRequest {
    pub slug: String,
    pub content: String,
    pub title: String,
    pub summary: Option<String>,
    pub public: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub author: String,
    pub published_at: DateTime<Utc>,
    pub public: bool,
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

#[derive(Deserialize, Serialize)]
pub struct CodeBlock {
    pub lang: String,
    pub code: String,
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub level: HeadingLevel,
    pub text: String,
    pub id: String,
}
