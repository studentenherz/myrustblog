use gloo_net::http::{Request, RequestBuilder};
use reqwest::StatusCode;
use serde::Serialize;

use crate::api_url;
use crate::utils::{cookies::*, get_current_host};

use common::LoginResponse;

pub struct AuthService;

pub enum AuthError {
    NetworkError,
    AuthenticationError,
    LoginError(String),
    RegistrationError,
    ConfirmationError,
    RegistrationConflict(String),
    LogoutError,
}

#[derive(Serialize)]
struct LoginForm<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(Serialize)]
struct RegistrationForm<'a> {
    username: &'a str,
    email: &'a str,
    password: &'a str,
    host: Option<String>,
}

#[derive(Serialize)]
struct UserConfirmation<'a> {
    confirmation_token: &'a str,
}

impl AuthService {
    pub async fn login(username: &str, password: &str) -> Result<(), AuthError> {
        let result = Request::post(&api_url!("/auth/login"))
            .json(&LoginForm { username, password })
            .unwrap()
            .send()
            .await;

        if let Ok(response) = result {
            match response.status() {
                200..=299 => {
                    if let Ok(res) = response.json::<LoginResponse>().await {
                        if set_cookie_with_attributes(
                            "_token",
                            &res.token,
                            CookieAttributes::new()
                                .max_age(res.max_age)
                                .path("/")
                                .same_site_strict()
                                .secure(),
                        )
                        .is_ok()
                            && set_cookie_with_attributes(
                                "_username",
                                &res.username,
                                CookieAttributes::new()
                                    .max_age(res.max_age)
                                    .path("/")
                                    .same_site_strict()
                                    .secure(),
                            )
                            .is_ok()
                        {
                            log::info!("Successfully loged in!");
                            return Ok(());
                        }
                    }
                }
                400..=499 => {
                    return Err(AuthError::LoginError(
                        "Incorrect username and password combination".to_string(),
                    ));
                }
                _ => {
                    return Err(AuthError::LoginError("Server error".to_string()));
                }
            }
        }

        // Handle network error
        log::error!("Error in the request");
        Err(AuthError::NetworkError)
    }

    pub async fn register(username: &str, email: &str, password: &str) -> Result<(), AuthError> {
        let host = get_current_host();

        let result = Request::post(&api_url!("/auth/register"))
            .json(&RegistrationForm {
                username,
                email,
                password,
                host,
            })
            .unwrap()
            .send()
            .await;

        if let Ok(response) = result {
            return match StatusCode::from_u16(response.status()).unwrap() {
                status_code if status_code.is_success() => {
                    log::info!("Successfully registered!");
                    Ok(())
                }
                StatusCode::INTERNAL_SERVER_ERROR => Err(AuthError::RegistrationError),
                StatusCode::CONFLICT => match response.text().await {
                    Ok(conflict) => {
                        log::error!("{} already in use", conflict);
                        Err(AuthError::RegistrationConflict(conflict))
                    }
                    _ => Err(AuthError::RegistrationError),
                },
                _ => Err(AuthError::RegistrationError),
            };
        }

        // Handle network error
        log::error!("Error in the request");
        Err(AuthError::NetworkError)
    }

    pub async fn confirm(token: &str) -> Result<(), AuthError> {
        let result = Request::post(&api_url!("/auth/confirm"))
            .json(&UserConfirmation {
                confirmation_token: token,
            })
            .unwrap()
            .send()
            .await;

        if let Ok(response) = result {
            return match StatusCode::from_u16(response.status()).unwrap() {
                status_code if status_code.is_success() => {
                    log::info!("User confirmation successful!");
                    Ok(())
                }
                StatusCode::INTERNAL_SERVER_ERROR => Err(AuthError::ConfirmationError),
                StatusCode::NOT_FOUND => Err(AuthError::ConfirmationError),
                _ => Err(AuthError::ConfirmationError),
            };
        }

        // Handle network error
        log::error!("Error in the request");
        Err(AuthError::NetworkError)
    }

    pub fn logout() -> Result<(), AuthError> {
        if delete_cookie("_token").is_ok() && delete_cookie("_username").is_ok() {
            return Ok(());
        }

        Err(AuthError::LogoutError)
    }

    pub fn _protected_get(url: &str) -> Result<RequestBuilder, AuthError> {
        if let Some(auth_token) = get_cookie("_token") {
            Ok(Request::get(url).header(
                reqwest::header::AUTHORIZATION.as_str(),
                &format!("Bearer {}", auth_token),
            ))
        } else {
            Err(AuthError::AuthenticationError)
        }
    }

    pub fn protected_post(url: &str) -> Result<RequestBuilder, AuthError> {
        if let Some(auth_token) = get_cookie("_token") {
            Ok(Request::post(url).header(
                reqwest::header::AUTHORIZATION.as_str(),
                &format!("Bearer {}", auth_token),
            ))
        } else {
            Err(AuthError::AuthenticationError)
        }
    }

    pub fn _protected_delete(url: &str) -> Result<RequestBuilder, AuthError> {
        if let Some(auth_token) = get_cookie("_token") {
            Ok(Request::delete(url).header(
                reqwest::header::AUTHORIZATION.as_str(),
                &format!("Bearer {}", auth_token),
            ))
        } else {
            Err(AuthError::AuthenticationError)
        }
    }
}
