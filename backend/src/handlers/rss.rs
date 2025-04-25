use actix_web::{web, HttpRequest, HttpResponse, Responder};
use common::PostsQueryParams;

use crate::{
    database::DBHandler,
    utils::{create_rss_feed, get_host_or},
    Config,
};

async fn generate_rss<T: DBHandler>(
    db_handler: web::Data<T>,
    config: &web::Data<Config>,
    per_page: u64,
    base_url: &str,
) -> impl Responder {
    if let Ok(latest_posts) = db_handler
        .get_posts(
            &PostsQueryParams {
                page: Some(1),
                per_page: Some(per_page),
                sort_by: Some("published_at".to_string()),
                sort_order: Some("desc".to_string()),
            },
            false,
        )
        .await
    {
        let feed = create_rss_feed(&latest_posts, &config, base_url);

        return HttpResponse::Ok()
            .content_type("application/rss+xml")
            .body(feed.to_string());
    }

    HttpResponse::InternalServerError().finish()
}

pub async fn rss_feed_handler<T: DBHandler>(
    request: HttpRequest,
    db_handler: web::Data<T>,
    config: web::Data<Config>,
) -> impl Responder {
    let base_url = get_host_or(&request, &config.WEBSITE_URL);
    generate_rss(db_handler, &config, 10, base_url).await
}

pub async fn rss_sitemap_handler<T: DBHandler>(
    request: HttpRequest,
    db_handler: web::Data<T>,
    config: web::Data<Config>,
) -> impl Responder {
    let base_url = get_host_or(&request, &config.WEBSITE_URL);
    generate_rss(db_handler, &config, 9999999, base_url).await
}

pub async fn robots(request: HttpRequest, config: web::Data<Config>) -> impl Responder {
    match std::fs::read_to_string("./dist/static/robots.txt") {
        Ok(robots_txt) => HttpResponse::Ok().content_type("text/plain").body(
            robots_txt
                + &format!(
                    "Sitemap: {}/sitemap",
                    get_host_or(&request, &config.WEBSITE_URL)
                ),
        ),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
