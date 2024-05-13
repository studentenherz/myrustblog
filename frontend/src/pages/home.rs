use pulldown_cmark::{html::push_html, Parser};
use yew::prelude::*;

use crate::{
    pages::Layout,
    utils::{set_description_meta_tag, set_title},
};

#[function_component(Home)]
pub fn home() -> Html {
    let content = use_state(|| include_str!("../../../README.md").to_string());

    {
        use_effect(|| {
            set_title("Home");
            set_description_meta_tag(
                "This is a blog webapp built as a Rust fullstack. The backend is made with \
                [Actix Web](https://actix.rs/) and the frontend with [Yew](https://yew.rs). \
                I'm also using [MongoDB](https://www.mongodb.com/) as the main database.",
            );
            || ()
        });
    }

    let parser = Parser::new(&*content);
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
