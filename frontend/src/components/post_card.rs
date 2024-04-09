use yew::prelude::*;
use yew_router::{history::History, prelude::*};

use crate::routes::AppRoute;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub title: AttrValue,
    #[prop_or_default]
    pub author: AttrValue,
    pub slug: AttrValue,
}

#[function_component(PostCard)]
pub fn post_card(props: &Props) -> Html {
    let slug = use_state(|| props.slug.to_string());

    let go_to_slug = move |_| {
        yew_router::history::BrowserHistory::new().push(
            AppRoute::Post {
                slug: slug.to_string(),
            }
            .to_path(),
        );
    };

    html! {
        <div class="post-card"  onclick={go_to_slug} >
            <h2> { &props.title } </h2>
            <p> { "by " } { &props.author } </p>
            { "see more..." }
        </div>
    }
}
