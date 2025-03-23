use std::sync::Arc;
use yew::prelude::*;

use crate::components::PostCard;
use common::Post;

#[derive(PartialEq, Properties)]
pub struct BlogProps {
    #[prop_or(0)]
    pub page: u64,
    #[prop_or(0)]
    pub pages: u64,
    #[prop_or_default]
    pub posts: Vec<Arc<Post>>,
}

#[function_component(Blog)]
pub fn blog(BlogProps { page, pages, posts }: &BlogProps) -> Html {
    let next_page_url = if page < pages {
        Some(format!("/blog?page={}", page + 1))
    } else {
        None
    };

    let prev_page_url = if *page > 1 {
        Some(format!("/blog?page={}", page - 1))
    } else {
        None
    };

    html! {
        <div class="blog-posts-list">
            <div class="posts-container">
                { for posts.iter().map(|post| html! {
                    <PostCard post={post.clone()} />
                }) }
            </div>
            <div class="posts-container-navigation">
                if let Some(prev_url) = prev_page_url {
                    <a href={ prev_url }>
                        <i class="icon-left icon"></i> { "Previous page" }
                    </a>
                } else {
                    <span class="disabled">
                        <i class="icon-left icon"></i> { "Previous page" }
                    </span>
                }
                <div>{ page } { " / " }  {pages} </div>
                if let Some(next_url) = next_page_url {
                    <a href={ next_url }>
                        { "Next page" } <i class="icon-right icon"></i>
                    </a>
                }
                else {
                    <span class="disabled">
                        { "Next page" } <i class="icon-right icon"></i>
                    </span>
                }
            </div>
        </div>
    }
}
