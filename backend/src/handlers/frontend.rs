use std::sync::Arc;

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use common::Post;
use common::PostsQueryParams;
use yew::ServerRenderer;

use crate::{
    database::DBHandler,
    utils::{parse_markdown, Highlighter},
};
use frontend::{Blog, BlogProps, Home, Layout, LayoutProps, PostPage, PostProps, UsernameAndRole};

async fn get_full_html<T: DBHandler>(
    content: &str,
    user_iden: &Option<Identity>,
    db_handler: &T,
) -> String {
    let index_html_string = include_str!("../../index.html");

    let mut user = None;
    if let Some(iden) = user_iden {
        if let Ok(username) = iden.id() {
            if let Ok(Some(full_user)) = db_handler.find_user(&username).await {
                user = Some(UsernameAndRole {
                    username: full_user.username,
                    role: full_user.role,
                });
            }
        }
    }

    let layout = ServerRenderer::<Layout>::with_props(|| LayoutProps {
        user,
        ..LayoutProps::default()
    })
    .hydratable(false)
    .render()
    .await;

    index_html_string
        .replace("</body>", &format!("{}</body>", layout))
        .replace("</main>", &format!("{}</main>", content))
}

pub async fn yew_home<T: DBHandler>(
    db_handler: web::Data<T>,
    user: Option<Identity>,
) -> impl Responder {
    let content = ServerRenderer::<Home>::new()
        .hydratable(false)
        .render()
        .await;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(get_full_html(&content, &user, db_handler.as_ref()).await)
}

pub async fn yew_blog<T: DBHandler>(
    db_handler: web::Data<T>,
    query: web::Query<PostsQueryParams>,
    user: Option<Identity>,
) -> impl Responder {
    let mut content = String::from("Sorry something went wrong");

    if let Ok(posts) = db_handler.get_posts(&query).await {
        let posts: Vec<Arc<Post>> = posts.into_iter().map(Arc::new).collect();
        if let Ok(pages) = db_handler
            .calculate_total_pages(query.per_page.unwrap_or(10))
            .await
        {
            content = ServerRenderer::<Blog>::with_props(move || BlogProps {
                page: query.page.unwrap_or(1),
                pages,
                posts,
            })
            .hydratable(false)
            .render()
            .await;
        }
    }

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(get_full_html(&content, &user, db_handler.as_ref()).await)
}

pub async fn yew_post<T: DBHandler>(
    db_handler: web::Data<T>,
    slug: web::Path<String>,
    highlighter: web::Data<Highlighter>,
    user_iden: Option<Identity>,
) -> impl Responder {
    let mut content = String::from("Sorry something went wrong");

    let mut user = None;
    if let Some(iden) = &user_iden {
        if let Ok(username) = iden.id() {
            if let Ok(Some(full_user)) = db_handler.find_user(&username).await {
                user = Some(UsernameAndRole {
                    username: full_user.username,
                    role: full_user.role,
                });
            }
        }
    }

    if let Ok(post) = db_handler.get_post(&slug).await {
        if let Some(post) = post {
            let (headers, html_string) = parse_markdown(&post.content, &highlighter);

            content = ServerRenderer::<PostPage>::with_props(move || PostProps {
                slug: slug.clone().into(),
                post: Arc::new(post),
                post_content: html_string.into(),
                headers,
                user,
            })
            .hydratable(false)
            .render()
            .await;
        } else {
            return HttpResponse::NotFound().finish();
        }
    }

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(get_full_html(&content, &user_iden, db_handler.as_ref()).await)
}
