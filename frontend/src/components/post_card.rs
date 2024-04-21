use std::rc::Rc;

use common::Post;
use pulldown_cmark::{Event, Parser};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub post: Rc<Post>,
}

const MAX_CONTENT_PREVIEW_LENGTH: usize = 150;

#[function_component(PostCard)]
pub fn post_card(Props { post }: &Props) -> Html {
    let mut content = String::new();
    let parser = Parser::new(&post.content);

    for event in parser {
        if let Event::Text(text) = event {
            content += &text;
        }
    }

    html! {
        <div class="post-card" >
            <h2> { &post.title } </h2>
            <p class="preview"> { &content[..MAX_CONTENT_PREVIEW_LENGTH.min(content.len())] } { "..." } </p>
            <div class="lower-strip">
                <p class="date" > { &post.published_at.format("%d %b %Y").to_string() } </p>
                <Link<AppRoute> to={AppRoute::Post { slug: post.slug.clone()}} > { "see more..." } </Link<AppRoute>>
            </div>
        </div>
    }
}
