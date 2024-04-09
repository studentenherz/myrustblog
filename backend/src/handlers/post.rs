use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;

use crate::{
    database::DBHandler,
    models::{Claims, Post, PostsQueryParams},
    utils::generate_unique_slug,
};
use common::{CreatePostRequest, PostCreatedResponse, UpdatePostRequest};

pub async fn create_post<T: DBHandler>(
    db_handler: web::Data<T>,
    claims: Claims,
    post: web::Json<CreatePostRequest>,
) -> impl Responder {
    if claims.role != "Admin" && claims.role != "Editor" {
        return HttpResponse::Unauthorized().finish();
    }

    if let Ok(slug) = generate_unique_slug(db_handler.as_ref(), &post.title).await {
        if db_handler
            .create_post(&Post {
                id: None,
                slug: slug.clone(),
                title: post.title.clone(),
                content: post.content.clone(),
                author: claims.sub,
                published_at: Utc::now(),
            })
            .await
            .is_ok()
        {
            return HttpResponse::Ok().json(PostCreatedResponse { slug });
        }
    }

    HttpResponse::InternalServerError().finish()
}

pub async fn update_post<T: DBHandler>(
    db_handler: web::Data<T>,
    claims: Claims,
    post: web::Json<UpdatePostRequest>,
) -> impl Responder {
    if claims.role != "Admin" && claims.role != "Editor" {
        return HttpResponse::Unauthorized().finish();
    }

    if db_handler
        .update_post(&post.slug, &post.content)
        .await
        .is_ok()
    {
        return HttpResponse::Ok().json(PostCreatedResponse {
            slug: post.slug.clone(),
        });
    }

    HttpResponse::InternalServerError().finish()
}

pub async fn delete_post<T: DBHandler>(
    db_handler: web::Data<T>,
    claims: Claims,
    slug: web::Path<String>,
) -> impl Responder {
    if claims.role != "Admin" && claims.role != "Editor" {
        return HttpResponse::Unauthorized().finish();
    }

    if let Ok(deleted_count) = db_handler.delete_post(&slug).await {
        return HttpResponse::Ok().json(deleted_count);
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
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}