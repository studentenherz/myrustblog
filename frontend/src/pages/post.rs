use std::rc::Rc;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::{history::History, Routable};

use crate::{
    pages::Layout,
    routes::AppRoute,
    services::api::ApiService,
    utils::{get_headers_and_html_with_ids, set_title, Header as MyHeader},
};
use common::Post;

#[derive(Debug, Default)]
pub struct PostPage {
    pub post: Post,
    pub post_content: String,
    pub headers: Vec<MyHeader>,
}

pub enum Msg {
    GetPost { post: Post },
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub slug: AttrValue,
}

impl Component for PostPage {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let get_post_cb = ctx.link().callback(|post| Msg::GetPost { post });
        let slug = ctx.props().slug.clone();

        spawn_local(async move {
            match ApiService::get_post(&slug).await {
                Ok(Some(post)) => {
                    set_title(&format!("Blog | {}", &post.title));
                    get_post_cb.emit(post);
                }
                Ok(None) => {
                    yew_router::history::BrowserHistory::new().replace(AppRoute::NotFound.to_path())
                }
                Err(_) => {
                    log::error!("Error")
                }
            }
        });

        Self::default()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Layout>
                    <div class={"post-container"}>
                        <div class="content-table">
                            <h2> { "Contents" } </h2>
                            <ul>
                                { self.headers.iter().map(|header| html! {
                                        <li class={format!("header-{:?}", header.level)}>
                                            <a href={format!("#{}", header.id.clone())}>  { header.text.clone() } </a>
                                        </li>
                                    }).collect::<Html>()
                                }
                            </ul>
                        </div>
                        <div class="post">
                            <div class="post-title">
                                <h1> { &self.post.title } </h1>
                                <div class="details">
                                    <p> { &self.post.author } </p>
                                    <p class="date"> { &self.post.published_at.format("%d %b %Y").to_string() } </p>
                                </div>
                            </div>
                            { Html::from_html_unchecked(self.post_content.clone().into()) }
                        </div>
                    </div>
                </Layout>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetPost { post } => {
                (self.headers, self.post_content) = get_headers_and_html_with_ids(&post.content);
                self.post = post;
            }
        }

        true
    }
}
