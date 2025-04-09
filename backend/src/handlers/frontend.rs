use std::sync::Arc;

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use common::{utils::get_summary, Post, PostsQueryParams};
use yew::ServerRenderer;

use crate::{
    database::DBHandler,
    utils::{parse_markdown, Highlighter},
};
use frontend::{Blog, BlogProps, Layout, LayoutProps, PostPage, PostProps, UsernameAndRole};

const MAX_SUMMARY_SIZE: usize = 200;

async fn get_full_html<T: DBHandler>(
    content: &str,
    user_iden: &Option<Identity>,
    db_handler: &T,
    title: &str,
    description: &str,
    slug: &str,
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
        .replace("</title>", &format!("{}</title>", title))
        .replace(
            r#"url" content="https://blog.studentenherz.dev""#,
            &format!(r#"url" content="https://blog.studentenherz.dev/{}""#, slug),
        )
        .replace(
            r#"title" content=""#,
            &format!(r#"title" content="{}"#, title),
        )
        .replace(
            r#"description" content=""#,
            &format!(r#"description" content="{}"#, description),
        )
        .replace("</body>", &format!("{}</body>", layout))
        .replace("</main>", &format!("{}</main>", content))
}

pub async fn yew_blog<T: DBHandler>(
    db_handler: web::Data<T>,
    query: web::Query<PostsQueryParams>,
    user: Option<Identity>,
) -> impl Responder {
    let mut content = String::from("Sorry something went wrong");
    let title = "Studentenherz's Blog";
    let description = "A blogging website made with Rust, using Yew and Actix Web.";

    if let Ok(posts) = db_handler.get_posts(&query).await {
        println!("Posts: {:?}", posts);
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
        .body(
            get_full_html(
                &content,
                &user,
                db_handler.as_ref(),
                title,
                description,
                "blog",
            )
            .await,
        )
}

pub async fn yew_post<T: DBHandler>(
    db_handler: web::Data<T>,
    slug: web::Path<String>,
    highlighter: web::Data<Highlighter>,
    user_iden: Option<Identity>,
) -> impl Responder {
    let mut content = String::from("Sorry something went wrong");
    let mut title = String::from("Studentenherz's Blog");
    let mut description =
        String::from("A blogging website made with Rust, using Yew and Actix Web.");

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
            title = post.title.clone();
            if let Some(summary) = &post.summary {
                description = summary[..std::cmp::min(MAX_SUMMARY_SIZE, summary.len())].to_string();
            } else {
                description = get_summary(&post.content, MAX_SUMMARY_SIZE);
            }
            {
                let slug = slug.clone();
                content = ServerRenderer::<PostPage>::with_props(move || PostProps {
                    slug: slug.into(),
                    post: Arc::new(post),
                    post_content: html_string.into(),
                    headers,
                    user,
                })
                .hydratable(false)
                .render()
                .await;
            }
        } else {
            return HttpResponse::NotFound().finish();
        }
    }

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            get_full_html(
                &content,
                &user_iden,
                db_handler.as_ref(),
                &title,
                &description,
                &format!("post/{}", slug),
            )
            .await,
        )
}
