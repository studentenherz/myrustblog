use std::rc::Rc;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::{history::History, Routable};

use crate::{
    components::PostComponent, pages::Layout, routes::AppRoute, services::api::ApiService,
    utils::set_title,
};
use common::Post;

#[derive(Debug, Default)]
pub struct PostPage {
    post: Rc<Post>,
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
                    <div class={"post"}>
                        <PostComponent post={Rc::clone(&self.post)} />
                    </div>
                </Layout>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetPost { post } => self.post = Rc::new(post),
        }

        true
    }
}
