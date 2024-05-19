use actix_web::{web, HttpResponse, Responder};
use common::PostsQueryParams;

use crate::{database::DBHandler, utils::create_rss_feed, Config};

async fn generate_rss<T: DBHandler>(
    db_handler: web::Data<T>,
    config: web::Data<Config>,
    per_page: u64,
) -> impl Responder {
    if let Ok(latest_posts) = db_handler
        .get_posts(&PostsQueryParams {
            page: Some(1),
            per_page: Some(per_page),
            sort_by: Some("published_at".to_string()),
            sort_order: Some("desc".to_string()),
        })
        .await
    {
        let feed = create_rss_feed(&latest_posts, &config);

        return HttpResponse::Ok()
            .content_type("application/rss+xml")
            .body(feed.to_string());
    }

    HttpResponse::InternalServerError().finish()
}

pub async fn rss_feed_handler<T: DBHandler>(
    db_handler: web::Data<T>,
    config: web::Data<Config>,
) -> impl Responder {
    generate_rss(db_handler, config, 10).await
}

pub async fn rss_sitemap_handler<T: DBHandler>(
    db_handler: web::Data<T>,
    config: web::Data<Config>,
) -> impl Responder {
    generate_rss(db_handler, config, 9999999).await
}

pub async fn robots(config: web::Data<Config>) -> impl Responder {
    match std::fs::read_to_string("./dist/static/robots.txt") {
        Ok(robots_txt) => HttpResponse::Ok()
            .content_type("text/plain")
            .body(robots_txt + &format!("Sitemap: {}/sitemap", config.WEBSITE_URL)),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
