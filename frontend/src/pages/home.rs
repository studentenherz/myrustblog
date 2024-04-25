use pulldown_cmark::{html::push_html, Parser};
use yew::prelude::*;

use crate::{pages::Layout, utils::set_title};

#[derive(Debug, Default)]
pub struct Home {
    pub content: String,
}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        set_title("Home");

        Self {
            content: include_str!("../../../README.md").to_string(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let parser = Parser::new(&self.content);

        let mut html_out = String::new();
        push_html(&mut html_out, parser);

        html! {
            <Layout>
                <div class="home-container">
                    <div class="home">
                        { Html::from_html_unchecked(html_out.into()) }
                    </div>
                </div>
            </Layout>
        }
    }
}
