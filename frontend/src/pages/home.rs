use pulldown_cmark::{html::push_html, Parser};
use yew::prelude::*;

use crate::pages::Layout;

#[derive(Debug, Default)]
pub struct Home {
    pub content: String,
}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            content: include_str!("../../../README.md").to_string(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let parser = Parser::new(&self.content);

        let mut html_out = String::new();
        push_html(&mut html_out, parser);

        html! {
            <Layout>
                <div class="post">
                    { Html::from_html_unchecked(html_out.into()) }
                </div>
            </Layout>
        }
    }
}
