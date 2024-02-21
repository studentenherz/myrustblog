use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{
    bson,
    bson::{doc, Bson},
    Client,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::models::{Claims, User, UserLogin as UserLoginForm, UserRegistration};

use common::LoginResponse;

pub async fn register_user(
    db_client: web::Data<Client>,
    user_info: web::Json<UserRegistration>,
) -> impl Responder {
    let users = db_client.database("rust_blog").collection("users");

    // Hash the password
    let hashed_password = match hash(&user_info.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Create user document & insert it into the database
    match users
        .insert_one(
            User {
                id: None, // to skip this param
                username: user_info.username.clone(),
                email: user_info.email.clone(),
                password: hashed_password.clone(),
                role: String::from("Reader"),
            },
            None,
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().body("User created successfully"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn login_user(
    db_client: web::Data<Client>,
    login_info: web::Json<UserLoginForm>,
) -> impl Responder {
    let users_collection = db_client.database("rust_blog").collection("users");

    // Query database for user
    if let Ok(Some(result)) = users_collection
        .find_one(doc! {"username": &login_info.username}, None)
        .await
    {
        if let Ok(user) = bson::from_bson::<User>(Bson::Document(result)) {
            // Verify password
            if verify(&login_info.password, &user.password).unwrap_or(false) {
                // Create JWT token
                let max_age: u64 = 60 * 60 * 24;
                let claims = Claims {
                    sub: user.email.to_string(),
                    role: user.role,
                    exp: get_expiration(max_age),
                };
                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret("secret".as_ref()),
                )
                .unwrap();
                return HttpResponse::Ok().json(LoginResponse { token, max_age });
                // Send back token to client
            }
        }
    }

    HttpResponse::Unauthorized().finish()
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
