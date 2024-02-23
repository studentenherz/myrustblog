use std::time::{Duration, SystemTime, UNIX_EPOCH};

use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::database::DBHandler;

use crate::models::{Claims, User, UserLogin as UserLoginForm, UserRegistration};
use common::LoginResponse;

pub async fn register_user<T: DBHandler>(
    db_handler: web::Data<T>,
    user_info: web::Json<UserRegistration>,
) -> impl Responder {
    if let Ok(Some(_)) = db_handler.find_user(&user_info.username).await {
        return HttpResponse::Conflict().body("username");
    }
    if let Ok(Some(_)) = db_handler.find_user_by_email(&user_info.email).await {
        return HttpResponse::Conflict().body("email");
    }

    // Hash the password
    let hashed_password = match hash(&user_info.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Create user document & insert it into the database
    match db_handler
        .insert_user(&User {
            id: None, // to skip this param
            username: user_info.username.clone(),
            email: user_info.email.clone(),
            password: hashed_password.clone(),
            role: String::from("Reader"),
        })
        .await
    {
        Ok(_) => HttpResponse::Ok().body("User created successfully"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn login_user<T: DBHandler>(
    db_handler: web::Data<T>,
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
                    &EncodingKey::from_secret("secret".as_ref()),
                ) {
                    HttpResponse::Ok().json(LoginResponse { token, max_age })
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
