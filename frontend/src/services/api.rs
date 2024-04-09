use crate::{api_url, services::auth::AuthService};
use common::{CreatePostRequest, Post, PostCreatedResponse, PostsQueryParams, UpdatePostRequest};
use reqwest::{Client, StatusCode};

#[derive(Debug)]
pub enum ApiError {
    Unauthorized,
    ServerInternalError,
    UnknownResponse,
    UnknownError,
    RequestError,
}

pub struct ApiService;

impl ApiService {
    pub async fn create_post(title: &str, content: &str) -> Result<String, ApiError> {
        if let Ok(builder) = AuthService::protected_post(api_url!("/post/create")) {
            if let Ok(response) = builder
                .json(&CreatePostRequest {
                    title: String::from(title),
                    content: String::from(content),
                })
                .send()
                .await
            {
                match response.status() {
                    x if x.is_success() => {
                        if let Ok(PostCreatedResponse { slug }) =
                            response.json::<PostCreatedResponse>().await
                        {
                            return Ok(slug);
                        }

                        return Err(ApiError::UnknownResponse);
                    }
                    x if x.is_server_error() => {
                        return Err(ApiError::ServerInternalError);
                    }
                    StatusCode::UNAUTHORIZED => {
                        return Err(ApiError::Unauthorized);
                    }
                    _ => {
                        return Err(ApiError::UnknownError);
                    }
                }
            }
        }

        Err(ApiError::RequestError)
    }

    pub async fn update_post(slug: &str, content: &str) -> Result<String, ApiError> {
        if let Ok(builder) = AuthService::protected_post(api_url!("/post/update")) {
            if let Ok(response) = builder
                .json(&UpdatePostRequest {
                    slug: String::from(slug),
                    content: String::from(content),
                })
                .send()
                .await
            {
                match response.status() {
                    x if x.is_success() => {
                        if let Ok(PostCreatedResponse { slug }) =
                            response.json::<PostCreatedResponse>().await
                        {
                            return Ok(slug);
                        }

                        return Err(ApiError::UnknownResponse);
                    }
                    x if x.is_server_error() => {
                        return Err(ApiError::ServerInternalError);
                    }
                    StatusCode::UNAUTHORIZED => {
                        return Err(ApiError::Unauthorized);
                    }
                    _ => {
                        return Err(ApiError::UnknownError);
                    }
                }
            }
        }

        Err(ApiError::RequestError)
    }

    pub async fn delete_post(slug: &str) -> Result<u64, ApiError> {
        if let Ok(builder) =
            AuthService::protected_delete(api_url!(format!("/post/delete/{}", slug)))
        {
            if let Ok(response) = builder.send().await {
                match response.status() {
                    x if x.is_success() => {
                        if let Ok(deleted_count) = response.json::<u64>().await {
                            return Ok(deleted_count);
                        }

                        return Err(ApiError::UnknownResponse);
                    }
                    x if x.is_server_error() => {
                        return Err(ApiError::ServerInternalError);
                    }
                    StatusCode::UNAUTHORIZED => {
                        return Err(ApiError::Unauthorized);
                    }
                    _ => {
                        return Err(ApiError::UnknownError);
                    }
                }
            }
        }

        Err(ApiError::RequestError)
    }

    pub async fn get_post(slug: &str) -> Result<Option<Post>, ApiError> {
        let client = Client::new();

        if let Ok(response) = client
            .get(api_url!(format!("/post/read/{}", slug)))
            .send()
            .await
        {
            match response.status() {
                x if x.is_success() => {
                    if let Ok(post) = response.json::<Post>().await {
                        return Ok(Some(post));
                    }

                    return Err(ApiError::UnknownResponse);
                }
                x if x.is_server_error() => {
                    return Err(ApiError::ServerInternalError);
                }
                StatusCode::NOT_FOUND => {
                    return Ok(None);
                }
                StatusCode::UNAUTHORIZED => {
                    return Err(ApiError::Unauthorized);
                }
                _ => {
                    return Err(ApiError::UnknownError);
                }
            }
        }

        Err(ApiError::RequestError)
    }

    pub async fn get_posts(
        page: Option<u64>,
        per_page: Option<u64>,
        sort_by: Option<String>,
        sort_order: Option<String>,
    ) -> Result<Vec<Post>, ApiError> {
        let client = Client::new();

        if let Ok(response) = client
            .get(api_url!("/post/get-list"))
            .query(&PostsQueryParams {
                page,
                per_page,
                sort_by,
                sort_order,
            })
            .send()
            .await
        {
            match response.status() {
                x if x.is_success() => {
                    if let Ok(posts) = response.json::<Vec<Post>>().await {
                        return Ok(posts);
                    }

                    return Err(ApiError::UnknownResponse);
                }
                x if x.is_server_error() => {
                    return Err(ApiError::ServerInternalError);
                }
                StatusCode::UNAUTHORIZED => {
                    return Err(ApiError::Unauthorized);
                }
                _ => {
                    return Err(ApiError::UnknownError);
                }
            }
        }

        Err(ApiError::RequestError)
    }
}
