use std::env;

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use dotenv::dotenv;
use middlewares::authorization;
use mongodb::{options::ClientOptions, Client};

mod handlers;
mod middlewares;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from .env file
    pretty_env_logger::init(); // Initialize logger

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let client_options = ClientOptions::parse(&database_url)
        .await
        .expect("Failed to parse client options");
    let client =
        Client::with_options(client_options).expect("Failed to initialize database client");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default()) // Logs every request
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE) // Specific headers allowed
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(Data::new(client.clone())) // MongoDB client
            .service(
                web::scope("/api/auth")
                    .service(
                        web::resource("/register")
                            .route(web::post().to(handlers::auth::register_user)),
                    )
                    .service(
                        web::resource("/login").route(web::post().to(handlers::auth::login_user)),
                    ),
            )
            .service(web::scope("/").wrap(authorization::Authorization))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
