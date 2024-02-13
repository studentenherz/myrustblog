use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use mongodb::{bson::doc, Client};

use crate::models::UserRegistration;

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
            doc! {
                "username": &user_info.username,
                "email": &user_info.email,
                "password": &hashed_password,
                "role": "Reader", // Default role
            },
            None,
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().body("User created successfully"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
