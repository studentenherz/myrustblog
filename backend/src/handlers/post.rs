use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;

use crate::{
    database::DBHandler,
    models::{Post, PostsQueryParams},
    utils::generate_unique_slug,
};
use common::{CreatePostRequest, GetPostsResponse, PostCreatedResponse, UpdatePostRequest};

pub async fn create_post<T: DBHandler>(
    db_handler: web::Data<T>,
    post: web::Json<CreatePostRequest>,
    user: Identity,
) -> impl Responder {
    if let Ok(user_id) = user.id() {
        if let Ok(slug) = generate_unique_slug(db_handler.as_ref(), &post.title).await {
            if db_handler
                .create_post(&Post {
                    id: None,
                    slug: slug.clone(),
                    title: post.title.clone(),
                    content: post.content.clone(),
                    author: user_id,
                    published_at: Utc::now(),
                })
                .await
                .is_ok()
            {
                return HttpResponse::Ok().json(PostCreatedResponse { slug });
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}

pub async fn update_post<T: DBHandler>(
    db_handler: web::Data<T>,
    post: web::Json<UpdatePostRequest>,
    user: Identity,
) -> impl Responder {
    if let Ok(_user_id) = user.id() {
        if db_handler
            .update_post(&post.slug, &post.content)
            .await
            .is_ok()
        {
            return HttpResponse::Ok().json(PostCreatedResponse {
                slug: post.slug.clone(),
            });
        }
    }

    HttpResponse::InternalServerError().finish()
}

pub async fn delete_post<T: DBHandler>(
    db_handler: web::Data<T>,
    slug: web::Path<String>,
    user: Identity,
) -> impl Responder {
    if let Ok(_user_id) = user.id() {
        if let Ok(deleted_count) = db_handler.delete_post(&slug).await {
            return HttpResponse::Ok().json(deleted_count);
        }
    }

    HttpResponse::InternalServerError().finish()
}

pub async fn get_post<T: DBHandler>(
    db_handler: web::Data<T>,
    slug: web::Path<String>,
) -> impl Responder {
    match db_handler.get_post(&slug).await {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_posts<T: DBHandler>(
    db_handler: web::Data<T>,
    query: web::Query<PostsQueryParams>,
) -> impl Responder {
    match db_handler.get_posts(&query).await {
        Ok(posts) => HttpResponse::Ok().json(GetPostsResponse {
            posts,
            pages: db_handler
                .calculate_total_pages(query.per_page.unwrap_or(10))
                .await,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
