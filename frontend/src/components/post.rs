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

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let parser = Parser::new(&ctx.props().post.content);

        let mut html_out = String::new();
        push_html(&mut html_out, parser);

        html! {
            { Html::from_html_unchecked(html_out.into()) }

        }
    }
}
