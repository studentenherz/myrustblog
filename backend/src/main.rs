use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::{Logger, NormalizePath},
    web::{self, Data},
    App, HttpServer,
};
use actix_web_lab::web::spa;

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
use utils::Highlighter;

create_env_struct! {
    Config {
        JWT_SECRET,
        DATABASE_URL,
        SMTP_SERVER,
        SMTP_USERNAME,
        SMTP_PASSWORD,
        NEW_USER_DEFAULT_ROLE,
        WEBSITE_URL,
        RSS_TITLE,
        RSS_DESCRIPTION
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from .env file
    pretty_env_logger::init(); // Initialize logger

    let config = Config::new();

    let db_handler = database::mongo::MongoDBHandler::new(&config.DATABASE_URL, "rust_blog")
        .await
        .expect("Error creating database handler");

    let emailer = Emailer::new(
        &config.SMTP_SERVER,
        &config.SMTP_USERNAME,
        &config.SMTP_PASSWORD,
    )
    .expect("Error loading env variables");

    emailer
        .test_connection()
        .await
        .expect("Connection test with SMTP server failed");

    let highlighter = Highlighter::new();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default()) // Logs every request
            .wrap(NormalizePath::trim())
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE) // Specific headers allowed
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(actix_web::middleware::Compress::default())
            .app_data(Data::new(db_handler.clone())) // MongoDB client
            .app_data(Data::new(emailer.clone())) // Emailer service
            .app_data(Data::new(config.clone())) // Config env variables
            .app_data(Data::new(highlighter.clone()))
            .service(web::resource("/rss").get(handlers::rss_feed_handler::<MongoDBHandler>))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .service(
                                web::resource("/register")
                                    .post(handlers::register_user::<MongoDBHandler>),
                            )
                            .service(
                                web::resource("/confirm")
                                    .post(handlers::confirm_user::<MongoDBHandler>),
                            )
                            .service(
                                web::resource("/login")
                                    .post(handlers::login_user::<MongoDBHandler>),
                            ),
                    )
                    .service(
                        web::scope("/post")
                            .service(
                                web::resource("/get-list")
                                    .get(handlers::get_posts::<MongoDBHandler>),
                            )
                            .service(
                                web::resource("/read/{slug}")
                                    .get(handlers::get_post::<MongoDBHandler>),
                            )
                            .service(
                                web::scope("")
                                    .wrap(authorization::Authorization::new(&config.JWT_SECRET))
                                    .service(
                                        web::resource("/create")
                                            .post(handlers::create_post::<MongoDBHandler>),
                                    )
                                    .service(
                                        web::resource("/update")
                                            .post(handlers::update_post::<MongoDBHandler>),
                                    )
                                    .service(
                                        web::resource("/delete/{slug}")
                                            .delete(handlers::delete_post::<MongoDBHandler>),
                                    ),
                            ),
                    )
                    .service(web::resource("/highlight").post(handlers::highlight_code)),
            )
            .service(
                spa()
                    .index_file("./dist/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("./dist")
                    .finish(),
            )
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
