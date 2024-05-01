use std::collections::HashMap;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, Document, HtmlElement};
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::{history::History, Routable};
use yewdux::prelude::*;

use crate::utils::{get_summary, set_description_meta_tag};
use crate::{
    pages::Layout,
    routes::AppRoute,
    services::api::ApiService,
    utils::{parse_markdown, set_title, AppState, Header as MyHeader, User},
};
use common::{CodeBlock, Post};

#[derive(Debug)]
pub struct PostPage {
    pub post: Post,
    pub post_content: String,
    pub headers: Vec<MyHeader>,
    _dispatch: Dispatch<AppState>,
    state: Rc<AppState>,
}

pub enum Msg {
    GetPost {
        post: Post,
    },
    StateChange(Rc<AppState>),
    HighlightCode {
        highlighted: HashMap<String, String>,
    },
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
                    let summary = get_summary(&post.content, 200);
                    set_description_meta_tag(&summary);
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

        let callback = ctx.link().callback(Msg::StateChange);
        let dispatch = Dispatch::<AppState>::global().subscribe_silent(callback);

        Self {
            post: Post::default(),
            post_content: Default::default(),
            headers: Default::default(),
            state: dispatch.get(),
            _dispatch: dispatch,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Layout>
                    <div class="post-title">
                        <h1> { &self.post.title } </h1>
                        <div class="details">
                            <p> { &self.post.author } </p>
                            <time datetime={self.post.published_at.to_rfc2822()}>
                                { &self.post.published_at.format("%d %b %Y").to_string() }
                            </time>
                        </div>
                    </div>
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
                            { Html::from_html_unchecked(self.post_content.clone().into()) }
                        </div>

                        if let Some(User{username: _, role}) = &self.state.user {
                            if role == "Admin" ||  role == "Editor" {
                                <div class="post-edit-bar">
                                    <Link<AppRoute> classes="clickable" to={AppRoute::Edit { slug: self.post.slug.clone() }}>
                                        <i class="fa-regular fa-pen-to-square icon"></i> { "Edit this post" }
                                    </Link<AppRoute>>
                                    // <Link<AppRoute> classes="clickable" to={AppRoute::Create}>
                                    //     <i class="fa-solid fa-trash icon"></i> { "Delete" }
                                    // </Link<AppRoute>>
                                </div>
                            }
                        }
                    </div>
                </Layout>
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetPost { post } => {
                let (headers, post_content, codeblocks) = parse_markdown(&post.content);
                self.headers = headers;
                self.post_content = post_content;
                self.post = post;

                let highlight_cb = ctx
                    .link()
                    .callback(|highlighted| Msg::HighlightCode { highlighted });

                spawn_local(async move {
                    match ApiService::highlight_code(codeblocks).await {
                        Ok(highlighted) => {
                            highlight_cb.emit(highlighted);
                        }
                        Err(_) => {
                            log::error!("Error")
                        }
                    }
                });
            }
            Msg::StateChange(state) => self.state = state,
            Msg::HighlightCode { highlighted } => {
                if let Some(window) = window() {
                    if let Some(document) = window.document() {
                        for (id, code) in highlighted.iter() {
                            if let Some(element) = document.get_element_by_id(&id) {
                                if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
                                    html_element.set_inner_html(code);
                                }
                            }
                        }
                    }
                }
            }
        }

        true
    }
}
