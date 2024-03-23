use std::env;

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::{Logger, NormalizePath, TrailingSlash},
    web::{self, Data},
    App, HttpResponse, HttpServer,
};

mod database;
mod handlers;
mod middlewares;
mod models;
mod services;
mod utils;

use database::mongo::MongoDBHandler;
use dotenv::dotenv;
use middlewares::authorization;
use services::email::Emailer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from .env file
    pretty_env_logger::init(); // Initialize logger

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_handler = database::mongo::MongoDBHandler::new(&database_url, "rust_blog")
        .await
        .expect("Error creating database handler");

    let emailer = Emailer::new().expect("Error loading env variables");

    emailer
        .test_connection()
        .await
        .expect("Connection test with SMTP server failed");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default()) // Logs every request
            .wrap(NormalizePath::new(TrailingSlash::MergeOnly))
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE) // Specific headers allowed
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(Data::new(db_handler.clone())) // MongoDB client
            .app_data(Data::new(emailer.clone())) //  Emailer service
            .service(
                web::scope("/api/auth")
                    .service(
                        web::resource("/register")
                            .route(web::post().to(handlers::auth::register_user::<MongoDBHandler>)),
                    )
                    .service(
                        web::resource("/confirm")
                            .route(web::post().to(handlers::auth::confirm_user::<MongoDBHandler>)),
                    )
                    .service(
                        web::resource("/login")
                            .route(web::post().to(handlers::auth::login_user::<MongoDBHandler>)),
                    ),
            )
            .service(web::scope("/").wrap(authorization::Authorization).service(
                web::resource("").route(web::get().to(|| async { HttpResponse::Ok().body("ok") })),
            ))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
