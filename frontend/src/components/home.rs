use pulldown_cmark::{html::push_html, Parser};
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let content = include_str!("../../../README.md").to_string();

    let parser = Parser::new(&content);
    let mut html_out = String::new();
    push_html(&mut html_out, parser);

    html! {
        <div class="home-container">
            <div class="home">
                { Html::from_html_unchecked(html_out.into()) }
            </div>
        </div>
    }
}
