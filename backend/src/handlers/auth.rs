use std::time::{Duration, SystemTime, UNIX_EPOCH};

use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{
    database::DBHandler,
    models::{
        Claims, UnconfirmedUser, UserConfirmation, UserLogin as UserLoginForm, UserRegistration,
    },
    services::email::Emailer,
    utils::generate_random_alphanumeric_str,
    Config,
};
use common::{utils::*, LoginResponse};

pub async fn register_user<T: DBHandler>(
    db_handler: web::Data<T>,
    emailer: web::Data<Emailer>,
    user_info: web::Json<UserRegistration>,
) -> impl Responder {
    if !is_valid_email(&user_info.email) {
        return HttpResponse::BadRequest().body("email");
    }

    let email = normalize_email(&user_info.email);

    if !is_valid_username(&user_info.username) {
        return HttpResponse::BadRequest().body("username");
    }

    if !is_valid_password(&user_info.password) {
        return HttpResponse::BadRequest().body("password");
    }

    if let Ok(Some(_)) = db_handler.find_user(&user_info.username).await {
        return HttpResponse::Conflict().body("username");
    }
    if let Ok(Some(_)) = db_handler.find_user_by_email(&email).await {
        return HttpResponse::Conflict().body("email");
    }
    if let Ok(Some(_)) = db_handler.find_unconfirmed_user(&user_info.username).await {
        return HttpResponse::Conflict().body("username");
    }
    if let Ok(Some(_)) = db_handler.find_unconfirmed_user_user_by_email(&email).await {
        return HttpResponse::Conflict().body("email");
    }

    // Hash the password
    let hashed_password = match hash(&user_info.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let confirmation_token = generate_random_alphanumeric_str(32);
    let host = match &user_info.host {
        Some(x) => x.clone(),
        _ => String::from("http://localhost"),
    };

    // Create user document & insert it into the database
    if db_handler
        .insert_unconfirmed_user(&UnconfirmedUser {
            confirmation_token: confirmation_token.clone(),
            host: host.clone(),
            created_at: Utc::now(),
            confirmed: false,
            username: user_info.username.clone(),
            email: email.clone(),
            password: hashed_password.clone(),
            role: String::from("Reader"),
        })
        .await
        .is_ok()
    {
        let link = format!("{}/confirm/{}", host, confirmation_token);
        if emailer.send_confirmation_email(&email, &link).await.is_ok() {
            return HttpResponse::Ok().body("User created successfully");
        }
    }

    HttpResponse::InternalServerError().finish()
}

pub async fn confirm_user<T: DBHandler>(
    db_handler: web::Data<T>,
    user_confirmation: web::Json<UserConfirmation>,
) -> impl Responder {
    // Create user document & insert it into the database
    if let Ok(user_option) = db_handler
        .confirm_user(&user_confirmation.confirmation_token)
        .await
    {
        match user_option {
            Some(user) => {
                if db_handler.insert_user(&user.into()).await.is_ok() {
                    return HttpResponse::Ok().body("Confirmation successful!");
                }
            }
            None => return HttpResponse::NotFound().finish(),
        }
    }
    HttpResponse::InternalServerError().finish()
}

pub async fn login_user<T: DBHandler>(
    db_handler: web::Data<T>,
    config: web::Data<Config>,
    login_info: web::Json<UserLoginForm>,
) -> impl Responder {
    match db_handler.find_user(&login_info.username).await {
        Ok(Some(user)) => {
            if verify(&login_info.password, &user.password).unwrap_or(false) {
                let max_age: u64 = 60 * 60 * 24;
                let claims = Claims {
                    sub: user.email.to_string(),
                    role: user.role,
                    exp: get_expiration(max_age),
                };

                if let Ok(token) = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(config.JWT_SECRET.as_ref()),
                ) {
                    HttpResponse::Ok().json(LoginResponse {
                        token,
                        username: login_info.username.clone(),
                        max_age,
                    })
                } else {
                    HttpResponse::InternalServerError().finish()
                }
            } else {
                HttpResponse::Unauthorized().finish()
            }
        }
        Ok(None) => HttpResponse::Unauthorized().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

fn get_expiration(seconds_from_now: u64) -> usize {
    let now = SystemTime::now();

    let expiration_time = now
        .checked_add(Duration::from_secs(seconds_from_now))
        .expect("Failed to calculate expiration time");

    expiration_time
        .duration_since(UNIX_EPOCH)
        .expect("Failed to convert to Unix timestamp")
        .as_secs() as usize
}
