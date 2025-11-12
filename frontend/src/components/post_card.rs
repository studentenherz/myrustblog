use std::sync::Arc;

use common::{utils::get_summary, Post};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub post: Arc<Post>,
}

const MAX_CONTENT_PREVIEW_LENGTH: usize = 150;

#[function_component(PostCard)]
pub fn post_card(Props { post }: &Props) -> Html {
    let generated_summary = get_summary(&post.content, MAX_CONTENT_PREVIEW_LENGTH);
    let summary = post.summary.as_deref().unwrap_or(&generated_summary);

    html! {
        <a class={ format!("post-card{}", if post.public { "" } else { " non-public" } ) } href={ format!("/post/{}", post.slug) } >
            <h2 style={ format!("view-transition-name: {}", post.slug) }> { &post.title } </h2>
            <p class="preview"> { summary } </p>
            <div class="lower-strip">
                <time datetime={post.published_at.to_rfc2822()}> { &post.published_at.format("%d %b %Y").to_string() } </time>
            </div>
        </a>
    }
}
