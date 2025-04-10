use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;

use crate::{database::DBHandler, models::PostsQueryParams, utils::generate_unique_slug};
use common::{CreatePostRequest, GetPostsResponse, Post, PostCreatedResponse, UpdatePostRequest};

pub async fn create_post<T: DBHandler>(
    db_handler: web::Data<T>,
    post: web::Json<CreatePostRequest>,
    user: Identity,
) -> impl Responder {
    if let Ok(user_id) = user.id() {
        if let Ok(db_result) = db_handler.find_user(&user_id).await {
            match db_result {
                Some(db_user) if db_user.role == "Admin" || db_user.role == "Editor" => {
                    if let Ok(slug) = generate_unique_slug(db_handler.as_ref(), &post.title).await {
                        if db_handler
                            .create_post(&Post {
                                slug: slug.clone(),
                                title: post.title.clone(),
                                content: post.content.clone(),
                                summary: post.summary.clone(),
                                author: user_id,
                                published_at: Utc::now(),
                                public: post.public,
                            })
                            .await
                            .is_ok()
                        {
                            return HttpResponse::Ok().json(PostCreatedResponse { slug });
                        }
                    }
                }
                _ => {
                    return HttpResponse::Unauthorized().finish();
                }
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
    if let Ok(user_id) = user.id() {
        if let Ok(db_result) = db_handler.find_user(&user_id).await {
            match db_result {
                Some(db_user) if db_user.role == "Admin" || db_user.role == "Editor" => {
                    if db_handler
                        .update_post(
                            &post.slug,
                            &post.content,
                            &post.title,
                            post.summary.as_deref(),
                            post.public,
                        )
                        .await
                        .is_ok()
                    {
                        return HttpResponse::Ok().json(PostCreatedResponse {
                            slug: post.slug.clone(),
                        });
                    }
                }
                _ => {
                    return HttpResponse::Unauthorized().finish();
                }
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}

pub async fn delete_post<T: DBHandler>(
    db_handler: web::Data<T>,
    slug: web::Path<String>,
    user: Identity,
) -> impl Responder {
    if let Ok(user_id) = user.id() {
        if let Ok(db_result) = db_handler.find_user(&user_id).await {
            match db_result {
                Some(db_user) if db_user.role == "Admin" || db_user.role == "Editor" => {
                    if let Ok(deleted_count) = db_handler.delete_post(&slug).await {
                        return HttpResponse::Ok().json(deleted_count);
                    }
                }
                _ => return HttpResponse::Unauthorized().finish(),
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}

pub async fn delete_post_and_redirect<T: DBHandler>(
    db_handler: web::Data<T>,
    slug: web::Path<String>,
    user: Identity,
) -> impl Responder {
    if let Ok(user_id) = user.id() {
        if let Ok(db_result) = db_handler.find_user(&user_id).await {
            match db_result {
                Some(db_user) if db_user.role == "Admin" || db_user.role == "Editor" => {
                    if let Ok(_deleted_count) = db_handler.delete_post(&slug).await {
                        return HttpResponse::Found()
                            .append_header(("location", "/"))
                            .finish();
                    }
                }
                _ => return HttpResponse::Unauthorized().finish(),
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}

pub async fn get_post<T: DBHandler>(
    db_handler: web::Data<T>,
    slug: web::Path<String>,
    user: Identity,
) -> impl Responder {
    let mut is_admin = false;
    if let Ok(user_id) = user.id() {
        if let Ok(db_result) = db_handler.find_user(&user_id).await {
            match db_result {
                Some(db_user) if db_user.role == "Admin" || db_user.role == "Editor" => {
                    is_admin = true;
                }
                _ => {}
            }
        }
    }

    match db_handler.get_post(&slug, is_admin).await {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_posts<T: DBHandler>(
    db_handler: web::Data<T>,
    query: web::Query<PostsQueryParams>,
    user: Identity,
) -> impl Responder {
    let mut is_admin = false;
    if let Ok(user_id) = user.id() {
        if let Ok(db_result) = db_handler.find_user(&user_id).await {
            match db_result {
                Some(db_user) if db_user.role == "Admin" || db_user.role == "Editor" => {
                    is_admin = true;
                }
                _ => {}
            }
        }
    }

    match db_handler.get_posts(&query, is_admin).await {
        Ok(posts) => HttpResponse::Ok().json(GetPostsResponse {
            posts,
            pages: db_handler
                .calculate_total_pages(query.per_page.unwrap_or(10))
                .await,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
