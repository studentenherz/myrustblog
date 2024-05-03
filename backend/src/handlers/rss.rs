use actix_web::{web, HttpResponse, Responder};
use common::PostsQueryParams;

use crate::{database::DBHandler, utils::create_rss_feed, Config};

pub async fn rss_feed_handler<T: DBHandler>(
    db_handler: web::Data<T>,
    config: web::Data<Config>,
) -> impl Responder {
    if let Ok(latest_posts) = db_handler
        .get_posts(&PostsQueryParams {
            page: Some(1),
            per_page: Some(10),
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
