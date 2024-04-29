use std::rc::Rc;

use common::Post;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{routes::AppRoute, utils::get_summary};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub post: Rc<Post>,
}

const MAX_CONTENT_PREVIEW_LENGTH: usize = 150;

#[function_component(PostCard)]
pub fn post_card(Props { post }: &Props) -> Html {
    let summary = get_summary(&post.content, MAX_CONTENT_PREVIEW_LENGTH);

    html! {
        <div class="post-card" >
            <h2> { &post.title } </h2>
            <p class="preview"> { summary } { "..." } </p>
            <div class="lower-strip">
                <p class="date" > { &post.published_at.format("%d %b %Y").to_string() } </p>
                <Link<AppRoute> to={AppRoute::Post { slug: post.slug.clone()}} > { "see more..." } </Link<AppRoute>>
            </div>
        </div>
    }
}
