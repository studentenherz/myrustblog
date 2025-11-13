use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    http::header,
    middleware::{Logger, NormalizePath},
    web::{self, Data},
    App, HttpServer,
};
use actix_web_lab::web::spa;

mod database;
mod handlers;
mod models;
mod services;
mod utils;

use database::mongo::MongoDBHandler;
use dotenv::dotenv;
use services::email::Emailer;
use utils::Highlighter;

create_env_struct! {
    Config {
        DATABASE_URL,
        SMTP_SERVER,
        SMTP_USERNAME,
        SMTP_PASSWORD,
        NEW_USER_DEFAULT_ROLE,
        WEBSITE_URL,
        RSS_TITLE,
        RSS_DESCRIPTION,
        REDIS_URL,
        FILE_UPLOAD_PATH,
        FILE_UPLOAD_URL
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

    if std::env::var("TEST_SMTP_CONNECTION").is_ok() {
        emailer
            .test_connection()
            .await
            .expect("Connection test with SMTP server failed");
    }

    let highlighter = Highlighter::new();

    let key = Key::generate();
    let redis_store = RedisSessionStore::new(&config.REDIS_URL)
        .await
        .expect("Can't connect to Redis");

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
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(redis_store.clone(), key.clone()))
            .app_data(Data::new(db_handler.clone())) // MongoDB client
            .app_data(Data::new(emailer.clone())) // Emailer service
            .app_data(Data::new(config.clone())) // Config env variables
            .app_data(Data::new(highlighter.clone()))
            .service(web::resource("/rss").get(handlers::rss_feed_handler::<MongoDBHandler>))
            .service(web::resource("/sitemap").get(handlers::rss_sitemap_handler::<MongoDBHandler>))
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
                    )
                    .service(web::resource("/upload").post(handlers::upload::<MongoDBHandler>))
                    .service(web::resource("/highlight").post(handlers::highlight_code)),
            )
            .service(web::redirect("/", "/blog"))
            .service(web::resource("/blog").get(handlers::yew_blog::<MongoDBHandler>))
            .service(web::resource("/post/{slug}").get(handlers::yew_post::<MongoDBHandler>))
            .service(web::resource("/logout").get(handlers::logout_user))
            .service(
                web::resource("/delete/{slug}")
                    .get(handlers::delete_post_and_redirect::<MongoDBHandler>),
            )
            .service(web::resource("/robots.txt").get(handlers::robots))
            .service(actix_files::Files::new(
                &config.FILE_UPLOAD_URL,
                &config.FILE_UPLOAD_PATH,
            ))
            .service(
                spa()
                    .index_file("./dist/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("./dist")
                    .finish(),
            )
            .service(actix_files::Files::new("/", "./dist"))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
