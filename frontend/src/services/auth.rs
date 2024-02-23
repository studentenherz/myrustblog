use reqwest::{Client, IntoUrl, RequestBuilder, StatusCode};
use serde::Serialize;

use crate::utils::cookies::{get_cookie, set_cookie_with_attributes, CookieAttributes};

use common::LoginResponse;

pub struct AuthService;

pub enum AuthError {
    NetworkError,
    LoginError,
    RegistrationError,
    RegistrationConflict(String),
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
}

impl AuthService {
    pub async fn login(username: &str, password: &str) -> Result<(), AuthError> {
        let client = Client::new();
        let result = client
            .post("http://localhost:8081/api/auth/login")
            .json(&LoginForm { username, password })
            .send()
            .await;

        if let Ok(response) = result {
            if response.status().is_success() {
                log::info!("Successfully loged in!");
                if let Ok(jwt) = response.json::<LoginResponse>().await {
                    if let Ok(_) = set_cookie_with_attributes(
                        "_token",
                        &jwt.token,
                        CookieAttributes::new()
                            .max_age(jwt.max_age)
                            .path("/")
                            .same_site_strict()
                            .secure(),
                    ) {
                        return Ok(());
                    }
                }
            }

            return Err(AuthError::LoginError);
        }

        // Handle network error
        log::error!("Error in the request");
        Err(AuthError::NetworkError)
    }

    pub async fn register(username: &str, email: &str, password: &str) -> Result<(), AuthError> {
        let client = Client::new();

        let result = client
            .post("http://localhost:8081/api/auth/register")
            .json(&RegistrationForm {
                username,
                email,
                password,
            })
            .send()
            .await;

        if let Ok(response) = result {
            return match response.status() {
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

    pub fn protected_get<U: IntoUrl>(url: U) -> Result<RequestBuilder, AuthError> {
        let client = Client::new();

        if let Some(auth_token) = get_cookie("_token") {
            Ok(client.get(url).header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", auth_token),
            ))
        } else {
            Err(AuthError::LoginError)
        }
    }

    pub fn _protected_post<U: IntoUrl>(url: U) -> Result<RequestBuilder, AuthError> {
        let client = Client::new();

        if let Some(auth_token) = get_cookie("_token") {
            Ok(client.post(url).header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", auth_token),
            ))
        } else {
            Err(AuthError::LoginError)
        }
    }
}
