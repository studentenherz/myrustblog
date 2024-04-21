use std::rc::Rc;

use pulldown_cmark::{html::push_html, Parser};
use yew::prelude::*;

use common::Post;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct PostComponentProps {
    pub post: Rc<Post>,
}

pub struct PostComponent {}

impl Component for PostComponent {
    type Message = ();
    type Properties = PostComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let post = &ctx.props().post;
        let parser = Parser::new(&post.content);

        let mut html_out = String::new();
        push_html(&mut html_out, parser);

        html! {
            <>
                <div class="post-title">
                    <h1> { &post.title } </h1>
                    <div class="details">
                        <p> { &post.author } </p>
                        <p class="date"> { &post.published_at.format("%d %b %Y").to_string() } </p>
                    </div>
                </div>
                { Html::from_html_unchecked(html_out.into()) }
            </>

        }
    }
}
