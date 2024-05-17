use std::collections::HashMap;

use crate::{api_url, services::auth::AuthService};
use common::{
    CodeBlock, CreatePostRequest, GetPostsResponse, Post, PostCreatedResponse, UpdatePostRequest,
};
use gloo_net::http::Request;
use reqwest::StatusCode;

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
        if let Ok(builder) = AuthService::protected_post(&api_url!("/post/create")) {
            if let Ok(response) = builder
                .json(&CreatePostRequest {
                    title: String::from(title),
                    content: String::from(content),
                })
                .unwrap()
                .send()
                .await
            {
                match StatusCode::from_u16(response.status()).unwrap() {
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

    pub async fn _update_post(slug: &str, content: &str, title: &str) -> Result<String, ApiError> {
        if let Ok(builder) = AuthService::protected_post(&api_url!("/post/update")) {
            if let Ok(response) = builder
                .json(&UpdatePostRequest {
                    slug: String::from(slug),
                    content: String::from(content),
                    title: String::from(title),
                })
                .unwrap()
                .send()
                .await
            {
                match StatusCode::from_u16(response.status()).unwrap() {
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

    pub async fn _delete_post(slug: &str) -> Result<u64, ApiError> {
        if let Ok(builder) =
            AuthService::_protected_delete(&api_url!(format!("/post/delete/{}", slug)))
        {
            if let Ok(response) = builder.send().await {
                match StatusCode::from_u16(response.status()).unwrap() {
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
        if let Ok(response) = Request::get(&api_url!(format!("/post/read/{}", slug)))
            .send()
            .await
        {
            match StatusCode::from_u16(response.status()).unwrap() {
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

    pub async fn _get_posts(
        page: Option<u64>,
        per_page: Option<u64>,
        sort_by: Option<String>,
        sort_order: Option<String>,
    ) -> Result<(Vec<Post>, Result<u64, ()>), ApiError> {
        let mut params = vec![];
        if let Some(val) = page {
            params.push(("page", format!("{}", val)));
        }
        if let Some(val) = per_page {
            params.push(("per_page", format!("{}", val)));
        }
        if let Some(val) = sort_by {
            params.push(("sort_by", val));
        }
        if let Some(val) = sort_order {
            params.push(("sort_order", val));
        }

        if let Ok(response) = Request::get(&api_url!("/post/get-list"))
            .query(params)
            .send()
            .await
        {
            match StatusCode::from_u16(response.status()).unwrap() {
                x if x.is_success() => {
                    if let Ok(response) = response.json::<GetPostsResponse>().await {
                        return Ok((response.posts, response.pages));
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

    pub async fn _highlight_code(
        code_blocks: HashMap<String, CodeBlock>,
    ) -> Result<HashMap<String, String>, ApiError> {
        if let Ok(response) = Request::post(&api_url!("/highlight/"))
            .json(&code_blocks)
            .unwrap()
            .send()
            .await
        {
            match StatusCode::from_u16(response.status()).unwrap() {
                x if x.is_success() => {
                    if let Ok(response) = response.json::<HashMap<String, String>>().await {
                        return Ok(response);
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
