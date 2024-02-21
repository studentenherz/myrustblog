use reqwest::Client;
use serde::Serialize;
pub struct AuthService;

pub enum AuthError {
    NetworkError,
    LoginError,
    RegistrationError,
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
                return Ok(());
            }

            log::error!("Error in the request, status = {}", response.status());
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
            if response.status().is_success() {
                log::info!("Successfully registered!");
                return Ok(());
            }

            log::error!("Error in the request, status = {}", response.status());
            return Err(AuthError::RegistrationError);
        }

        // Handle network error
        log::error!("Error in the request");
        Err(AuthError::NetworkError)
    }
}
