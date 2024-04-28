use std::rc::Rc;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::{history::History, Routable};
use yewdux::prelude::*;

use crate::{
    pages::Layout,
    routes::AppRoute,
    services::api::ApiService,
    utils::{get_headers_and_html_with_ids, set_title, AppState, Header as MyHeader, User},
};
use common::Post;

#[derive(Debug)]
pub struct PostPage {
    pub post: Post,
    pub post_content: String,
    pub headers: Vec<MyHeader>,
    _dispatch: Dispatch<AppState>,
    state: Rc<AppState>,
}

pub enum Msg {
    GetPost { post: Post },
    StateChange(Rc<AppState>),
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
                            <p class="date"> { &self.post.published_at.format("%d %b %Y").to_string() } </p>
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
                                    <Link<AppRoute> classes="clickable" to={AppRoute::Create}>
                                        <i class="fa-regular fa-pen-to-square icon"></i> { "Edit" }
                                    </Link<AppRoute>>
                                    <Link<AppRoute> classes="clickable" to={AppRoute::Create}>
                                        <i class="fa-solid fa-trash icon"></i> { "Delete" }
                                    </Link<AppRoute>>
                                </div>
                            }
                        }
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
            Msg::StateChange(state) => self.state = state,
        }

        true
    }
}
